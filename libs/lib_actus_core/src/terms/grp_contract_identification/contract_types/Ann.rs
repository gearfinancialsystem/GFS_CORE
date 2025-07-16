use std::{rc::Rc, collections::HashSet, fmt};
use std::ops::Deref;
use std::str::FromStr;
use chrono::Days;

use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;

use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::util::RedemptionUtils::RedemptionUtils;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

use crate::functions::{
    ann::stf::STF_PRD_ANN::STF_PRD_ANN,
    lam::pof::{
        POF_IP_LAM::POF_IP_LAM, POF_IPCB_LAM::POF_IPCB_LAM, POF_PRD_LAM::POF_PRD_LAM, POF_TD_LAM::POF_TD_LAM,
    },
    lam::stf::{
        STF_FP_LAM::STF_FP_LAM, STF_IED_LAM::STF_IED_LAM, STF_IPBC_LAM::STF_IPCB_LAM, STF_IPCI2_LAM::STF_IPCI2_LAM, STF_IPCI_LAM::STF_IPCI_LAM,
        STF_MD_LAM::STF_MD_LAM, STF_PRD_LAM::STF_PRD_LAM, STF_RR_LAM::STF_RR_LAM, STF_RRF_LAM::STF_RRF_LAM, STF_SC_LAM::STF_SC_LAM
    },
    nam::pof::POF_PR_NAM::POF_PR_NAM,
    nam::stf::{STF_PR2_NAM::STF_PR2_NAM, STF_PR_NAM::STF_PR_NAM},
    pam::pof::{POF_FP_PAM::POF_FP_PAM, POF_IED_PAM::POF_IED_PAM, POF_IPCI_PAM::POF_IPCI_PAM, POF_MD_PAM::POF_MD_PAM, POF_RR_PAM::POF_RR_PAM, POF_SC_PAM::POF_SC_PAM,},
    pam::stf::{STF_IP_PAM::STF_IP_PAM, STF_TD_PAM::STF_TD_PAM}
};

use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

pub struct ANN;

impl TraitContractModel for ANN {

    fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events : Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new(); // A revoir
        let maturity = Self::maturity(model);

        // Initial exchange (IED)
        // ::<InitialExchangeDate, InitialExchangeDate>
        let e : ContractEvent<InitialExchangeDate, InitialExchangeDate>= EventFactory::create_event(
            &model.initial_exchange_date.clone(),
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption (MD)
        // ::<MaturityDate, MaturityDate>
        let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption schedule (PR)
        let mut stf: Rc<dyn TraitStateTransitionFunction>;
        if model.interest_calculation_base.clone().unwrap() != InterestCalculationBase::NT(NT) {
            stf = Rc::new(STF_PR_NAM)
        } else {
            stf = Rc::new(STF_PR2_NAM)
        };
        println!("{:?}", &model.cycle_anchor_date_of_principal_redemption);
        let a = &ScheduleFactory::<
            CycleAnchorDateOfPrincipalRedemption,
            MaturityDate,
            CycleOfPrincipalRedemption,
            IsoDatetime
        >::create_schedule(
            &model.cycle_anchor_date_of_principal_redemption,
            &Some(maturity.clone()),
            &model.cycle_of_principal_redemption,
            &model.end_of_month_convention.clone(),
            Some(false),
        );
        let es = EventFactory::create_events(
            a,
            &EventType::PR,
            &model.currency,
            Some(Rc::new(POF_PR_NAM)),
            Some(stf),
            &model.business_day_adjuster.clone(),
            &model.contract_id,
        );
        events.extend(es);

        // Initial principal redemption fixing event (PRF)
        if model.next_principal_redemption_payment.is_none() {
            let e: ContractEvent<CycleAnchorDateOfPrincipalRedemption,
                CycleAnchorDateOfPrincipalRedemption> = EventFactory::create_event(
                &CycleAnchorDateOfPrincipalRedemption::new((model.cycle_anchor_date_of_principal_redemption.clone().map(|d|
                    d.value() - Days::new(1))).unwrap()).ok(),
                &EventType::PRF,
                &model.currency,
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRD_ANN)),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Fees (FP)
        if model.cycle_of_fee.is_some() {
            events.extend(EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &Some(maturity.clone()),
                    &model.cycle_of_fee,
                    &model.end_of_month_convention,
                    Some(true),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            ));
        }

        // Purchase (PRD)
        if let Some(purchase_date) = model.purchase_date.clone() {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Interest payment related events (IP)
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        if model.nominal_interest_rate.is_some() &&
            (model.cycle_of_interest_payment.is_some() ||
                model.cycle_anchor_date_of_interest_payment.is_some()) {
            let mut interest_events = EventFactory::create_events(
                &ScheduleFactory::<CycleAnchorDateOfInterestPayment,
                MaturityDate,
                CycleOfInterestPayment,
                IsoDatetime>::create_schedule(
                    &model.cycle_anchor_date_of_interest_payment,
                    &Some(maturity.clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(true),
                ),
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );

            if model.cycle_anchor_date_of_interest_payment.clone().unwrap().value()
                != model.cycle_anchor_date_of_principal_redemption.clone().unwrap().value()
                    || model.cycle_of_interest_payment.clone().unwrap().value()
                != model.cycle_of_principal_redemption.clone().unwrap().value() {

                //let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap()).unwrap();
                let prcl = model.cycle_of_principal_redemption.clone().unwrap().value().extract_period().unwrap();
                let pranxm = model.cycle_anchor_date_of_principal_redemption.clone().unwrap().value() - prcl;
                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time.clone().unwrap() >= pranxm));

                let ipanxm = EventFactory::create_event(
                    &Some(pranxm),
                    &EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    &model.business_day_adjuster.clone(),
                    &model.contract_id,
                );
                interest_events.insert(ipanxm);

                let s = ScheduleFactory::<
                CycleAnchorDateOfPrincipalRedemption,
                    MaturityDate,
                    CycleOfPrincipalRedemption,
                    IsoDatetime
                >::create_schedule(
                    &model.cycle_anchor_date_of_principal_redemption,
                    &Some(maturity.clone()),
                    &model.cycle_of_principal_redemption,
                    &model.end_of_month_convention,
                    Some(true),
                );

                let es = EventFactory::create_events(
                    &s,
                    &EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    &model.business_day_adjuster.clone(),
                    &model.contract_id,
                );
                interest_events.extend(es);
            }

            if let Some(capitalization_end_date) = model.capitalization_end_date.clone() {
                let capitalization_end = EventFactory::create_event(
                    &Some(capitalization_end_date),
                    &EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci.clone()),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.retain(|e| !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time));
                interest_events.insert(capitalization_end.to_iso_datetime_event());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.extend(interest_events);
        }
        else if model.capitalization_end_date.is_some() {
            let e: ContractEvent<CapitalizationEndDate, CapitalizationEndDate> = EventFactory::create_event(
                &model.capitalization_end_date.clone(),
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                &model.business_day_adjuster.clone(),
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

        }
        else if model.cycle_of_interest_payment.is_none() && model.cycle_anchor_date_of_interest_payment.is_none() {

            let s = ScheduleFactory::<
                CycleAnchorDateOfPrincipalRedemption,
                MaturityDate,
                CycleOfPrincipalRedemption,
                IsoDatetime
            >::create_schedule(
                &model.cycle_anchor_date_of_principal_redemption,
                &Some(maturity.clone()),
                &model.cycle_of_principal_redemption,
                &model.end_of_month_convention,
                Some(true),
            );
            let interest_events = EventFactory::create_events(
                &s,
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(interest_events);
        }

        // Interest calculation base (IPCB)
        if let Some(interest_calculation_base) = &model.interest_calculation_base {
            if interest_calculation_base.clone() == InterestCalculationBase::NTL(NTL) {
                let s = ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_calculation_base.clone(),
                    &Some(maturity.clone()),
                    &model.cycle_of_interest_calculation_base.clone(),
                    &model.end_of_month_convention,
                    Some(false),
                );
                let es = EventFactory::create_events(
                    &s,
                    &EventType::IPCB,
                    &model.currency,
                    Some(Rc::new(POF_IPCB_LAM)),
                    Some(Rc::new(STF_IPCB_LAM)),
                    &model.clone().business_day_adjuster,
                    &model.contract_id,
                );
                events.extend(es);
            }
        }

        // Rate reset events (RR)
        let s = ScheduleFactory::<CycleAnchorDateOfRateReset,
            MaturityDate,
            CycleOfRateReset,
            IsoDatetime
        >::create_schedule(
            &model.cycle_anchor_date_of_rate_reset,
            &Some(maturity.clone()),
            &model.cycle_of_rate_reset,
            &model.end_of_month_convention,
            Some(false),
        );
        let mut rate_reset_events = EventFactory::create_events(
            &s,
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_LAM)),
            &model.clone().business_day_adjuster,
            &model.contract_id,
        );
        // adapt fixed rate reset event
        if let Some(next_reset_rate) = model.next_reset_rate.clone() {
            let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                &model.status_date.clone(),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            let mut fixed_eventa =
                rate_reset_events.clone().iter()
                    .find(|e| e > &&status_event.to_iso_datetime_event()).unwrap().clone();
            fixed_eventa.fstate = Some(Rc::new(STF_RRF_LAM));
            fixed_eventa.event_type = EventType::RRF;
            rate_reset_events.insert(fixed_eventa.clone());


        }
        // add all rate reset events
        events.extend(rate_reset_events.clone());

        // add all rate reset events
        let prf_schedule: HashSet<_> = rate_reset_events.clone().iter()
            .map(|e| e.event_time.unwrap()).collect();
        if !prf_schedule.is_empty() {
            let es = EventFactory::create_events(
                &prf_schedule,
                &EventType::PRF,
                &model.currency,
                Some(Rc::new(POF_RR_PAM)),
                Some(Rc::new(STF_PRD_ANN)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(es);
        }

        // scaling (if specified)
        if let Some(scaling_effect) = &model.scaling_effect {
            if scaling_effect.to_string().contains('I') || scaling_effect.to_string().contains('N') {
                let s = ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_scaling_index.clone(),
                    &Some(maturity.clone()),
                    &model.cycle_of_scaling_index,
                    &model.end_of_month_convention,
                    Some(false),
                );
                let es = EventFactory::create_events(
                    &s,
                    &EventType::SC,
                    &model.currency,
                    Some(Rc::new(POF_SC_PAM)),
                    Some(Rc::new(STF_SC_LAM)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );
                events.extend(es);
            }
        }

        // Termination event (TD)
        if let Some(termination_date) = model.termination_date.clone() {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &Some(termination_date),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_LAM)),
                Some(Rc::new(STF_TD_PAM)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e <= &termination.to_iso_datetime_event());
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date,
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e >= &status_event.to_iso_datetime_event());

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e <= &to_event);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        Ok(events)
    }

    fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>, model: &ContractModel, observer: &DataObserver)
        -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let _maturity = &model.maturity_date.clone();
        let mut states = Self::init_state_space(model, observer, _maturity).expect("Failed to initialize state space");
        let mut events = events;

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        for event in &mut events {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention,
                &model.clone().business_day_adjuster.unwrap(),
            );
        }

        if let Some(purchase_date) = model.purchase_date.clone() {
            let purchase_event: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date),
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e|
                !(e.event_type != EventType::AD && e.compare_to(&purchase_event.to_iso_datetime_event()) == -1) );
        }

        Ok(events)
    }

    fn init_state_space(model: &ContractModel, _observer: &DataObserver, _maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();
        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = model.interest_scaling_multiplier.clone();
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date.clone();
        states.maturity_date = Some(Self::maturity(model));

        if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok(); //Some(0.0);
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok();// Some(0.0);
            states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(0.0).ok(); //Some(0.0);
        } else {
            states.notional_principal = NotionalPrincipal::new(&model.contract_role.clone().unwrap().role_sign() * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = Some(model.nominal_interest_rate.clone().unwrap());

            if model.interest_calculation_base == Some(InterestCalculationBase::NT(NT)) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::from_str(states.notional_principal.clone().unwrap().value().to_string().as_str()).ok();
            } else {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(&model.clone().contract_role.clone().unwrap().role_sign() * model.interest_calculation_base_amount.clone().unwrap().value()).ok();
            }
        }

        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = AccruedInterest::new(0.0).ok();
        } else if model.accrued_interest.is_some() {
            states.accrued_interest = AccruedInterest::new(&model.contract_role.clone().unwrap().role_sign() * model.accrued_interest.clone().unwrap().value()).ok();
        } else {
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.clone().business_day_adjuster.unwrap();

            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone(),
                &states.maturity_date.clone(),
                &model.cycle_of_interest_payment,
                &model.end_of_month_convention,
                Some(true),
            ).into_iter().collect();

            ip_schedule.sort();

            let date_earlier_than_t0: Vec<_> = ip_schedule.iter().filter(|&&date| date < states.status_date.clone().unwrap().value()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accrued_interest = AccruedInterest::new(day_counter.day_count_fraction(
                time_adjuster.shift_sc(*t_minus),
                time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
            ) * states.notional_principal.clone().unwrap().value() * states.nominal_interest_rate.clone().unwrap().value()).ok();
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued.clone();
        }

        if model.next_principal_redemption_payment.is_none() {
            if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
                // Fixed at initial PRF event
            } else {
                states.next_principal_redemption_payment = NextPrincipalRedemptionPayment::new(RedemptionUtils::redemptionAmount(model, &states)).ok();
            }
        } else {
            states.next_principal_redemption_payment = model.next_principal_redemption_payment.clone();
        }

        Ok(states)
    }
}

impl ANN {
    fn maturity(model: &ContractModel) -> MaturityDate {
        if let Some(maturity_date) = model.maturity_date.clone() {
            let a = maturity_date.clone().deref().clone();
            return a;
        }

        if let Some(amortization_date) = model.amortization_date.as_ref() {
            let a = amortization_date.clone().value().to_string();
            let b = MaturityDate::from_str(a.as_str()).unwrap();
            return b;
        }

        let t0 = model.status_date.as_ref().unwrap();
        let pranx = model.cycle_anchor_date_of_principal_redemption.as_ref().unwrap();
        let ied = model.initial_exchange_date.as_ref().unwrap();
        let copr = model.cycle_of_principal_redemption.as_ref().unwrap();
        let prcl = copr.clone().value().extract_period().unwrap();

        let last_event = if pranx.value() >= t0.value() {
            pranx.value()
        } else if ied.value() + prcl.clone() > t0.value() {
            ied.value() + prcl.clone()
        } else {
            let mut previous_events: Vec<IsoDatetime> = ScheduleFactory::
            <CycleAnchorDateOfPrincipalRedemption,
                StatusDate,
                CycleOfPrincipalRedemption,
                IsoDatetime>::create_schedule(
                &model.cycle_anchor_date_of_principal_redemption,
                &Some(t0.clone()),
                &model.cycle_of_principal_redemption,
                &model.end_of_month_convention,
                Some(false)
            ).into_iter().collect();

            previous_events.retain(|&d| d > t0.value());
            previous_events.sort();
            *previous_events.last().unwrap()
        };

        let time_from_last_event_plus_one_cycle = model.day_count_convention.as_ref().unwrap().day_count_fraction(last_event.value(), last_event + prcl.clone());
        let redemption_per_cycle = model.next_principal_redemption_payment.clone().unwrap().value() - (time_from_last_event_plus_one_cycle * model.nominal_interest_rate.clone().unwrap().value() * model.notional_principal.clone().unwrap().value());
        let remaining_periods = ((model.notional_principal.clone().unwrap().value() / redemption_per_cycle).ceil() - 1.0) as i32;

        MaturityDate::new(model.business_day_adjuster.clone().unwrap()
            .shift_bd( &(last_event.clone() + prcl.multiplied_by(remaining_periods))   )).ok().unwrap()
    }

}

impl fmt::Display for ANN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ANN")
    }
}


