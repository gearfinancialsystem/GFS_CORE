use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;

use crate::state_space::StateSpace::StateSpace;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::functions::clm::pof::POF_IED_CLM::POF_IED_CLM;
use crate::functions::clm::pof::POF_IP_CLM::POF_IP_CLM;
use crate::functions::clm::stf::STF_IP_CLM::STF_IP_CLM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::stf::STF_FP_PAM::STF_FP_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::pam::stf::STF_IPCI_PAM::STF_IPCI_PAM;
use crate::functions::pam::stf::STF_MD_PAM::STF_MD_PAM;
use crate::functions::pam::stf::STF_RR_PAM::STF_RR_PAM;
use crate::functions::pam::stf::STF_RRF_PAM::STF_RRF_PAM;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

pub struct CLM;

impl TraitContractModel for CLM {
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {

        let mut events : Vec<ContractEvent<IsoDatetime, IsoDatetime>>= Vec::new();

        // Determine maturity of the contract
        let maturity = Self::maturity(model, &to.unwrap());

        // Initial exchange
        let e = EventFactory::<InitialExchangeDate, InitialExchangeDate>::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_CLM)),
            Some(Rc::new(STF_IED_PAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Interest payment event
        let e = EventFactory::<MaturityDate, MaturityDate>::create_event(
            &Some(maturity.clone()),
            &EventType::IP,
            &model.currency,
            Some(Rc::new(POF_IP_CLM)),
            Some(Rc::new(STF_IP_CLM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption
        let e:ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_PAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Interest payment capitalization (if specified)
        if model.cycle_of_interest_payment.is_some() {
            let cycle_anchor_date_of_interest_payment = if model.cycle_anchor_date_of_interest_payment.is_none() {
                //model.initial_exchange_date.clone().unwrap() + CycleUtils::parse_period(&model.cycle_of_interest_payment.clone().unwrap()).unwrap()
                CycleAnchorDateOfInterestPayment::new({
                    model.initial_exchange_date.clone().unwrap().value() + model.cycle_of_interest_payment.clone().unwrap().value().extract_period().unwrap()
                }).ok(
                ).unwrap()
            } else {
                model.cycle_anchor_date_of_interest_payment.clone().unwrap()
            };

            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &Some(cycle_anchor_date_of_interest_payment),
                    &Some(maturity.clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(false),
                ),
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(Rc::new(STF_IPCI_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::<
            CycleAnchorDateOfRateReset,
                MaturityDate,
                CycleOfRateReset,
                IsoDatetime
            >::create_schedule(
                &model.cycle_anchor_date_of_rate_reset,
                &Some(maturity.clone()),
                &model.cycle_of_rate_reset,
                &model.end_of_month_convention,
                Some(false),
            ),
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                &model.status_date,
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort();

            if let Some(fixed_event) = sorted_events.iter().find(|&&e| e.compare_to(&status_event.to_iso_datetime_event()) == 1).cloned() {
                let mut fixed_event_clone = fixed_event.clone();
                fixed_event_clone.set_f_state_trans(Some(Rc::new(STF_RRF_PAM)));
                fixed_event_clone.chg_event_type(EventType::RRF);
                rate_reset_events.insert(fixed_event_clone);
            }
        }

        events.extend(rate_reset_events);

        // Fees (if specified)
        if model.cycle_of_fee.is_some() {
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &Some(maturity.clone()),
                    &model.cycle_of_fee,
                    &model.end_of_month_convention,
                    Some(false),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&status_event.to_iso_datetime_event()) != -1);

        // Remove all post to-date events
        let to_event: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&to_event) != 1);

        // Sort the events according to their time of occurrence
        events.sort();

        Ok(events)
    }

    fn apply(
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &DataObserver,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        // Initialize state space per status date
        let _maturity = &model.maturity_date.clone();
        let mut states = Self::init_state_space(model, observer, _maturity).expect("Failed to initialize state_space");

        // Sort the events according to their time sequence
        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.clone(),
                &model.business_day_adjuster.clone().unwrap(),
            );
        }

        Ok(events)
    }



    fn init_state_space(model: &ContractModel, _observer: &DataObserver, _maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        states.interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();
        states.status_date = model.status_date.clone();

        if model.initial_exchange_date.clone().unwrap().value() <= model.status_date.clone().unwrap().value() {
            let role_sign = model.contract_role.as_ref().unwrap().role_sign();
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
            states.accrued_interest = AccruedInterest::new(role_sign * model.accrued_interest.clone().unwrap().value()).ok();
            states.fee_accrued = model.fee_accrued.clone();
        }

        Ok(states)
    }
}

impl CLM {
    fn maturity(model: &ContractModel, to: &IsoDatetime) -> MaturityDate {
        MaturityDate::from_str(model.maturity_date.clone().unwrap().value().to_string().as_str()).unwrap()
    }
}

impl fmt::Display for CLM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLM")
    }
}