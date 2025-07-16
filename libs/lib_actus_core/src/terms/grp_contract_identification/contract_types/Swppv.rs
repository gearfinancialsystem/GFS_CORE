
use std::fmt;
use std::rc::Rc;
use crate::attributes::ContractModel::ContractModel;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};

use crate::functions::fxout::pof::POF_PRD_FXOUT::POF_PRD_FXOUT;
use crate::functions::fxout::pof::POF_TD_FXOUT::POF_TD_FXOUT;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::swppv::pof::POF_IED_SWPPV::POF_IED_SWPPV;
use crate::functions::swppv::pof::POF_IP_SWPPV::POF_IP_SWPPV;
use crate::functions::swppv::pof::POF_IPFIx_SWPPV::POF_IPFix_SWPPV;
use crate::functions::swppv::pof::POF_IPFloat_SWPPV::POF_IPFloat_SWPPV;
use crate::functions::swppv::pof::POF_MD_SWPPV::POF_MD_SWPPV;
use crate::functions::swppv::stf::STF_IED_SWPPV::STF_IED_SWPPV;
use crate::functions::swppv::stf::STF_IP_SWPPV::STF_IP_SWPPV;
use crate::functions::swppv::stf::STF_IPFix_SWPPV::STF_IPFix_SWPPV;
use crate::functions::swppv::stf::STF_IPFloat_SWPPV::STF_IPFloat_SWPPV;
use crate::functions::swppv::stf::STF_MD_SWPPV::STF_MD_SWPPV;
use crate::functions::swppv::stf::STF_PRD_SWPPV::STF_PRD_SWPPV;
use crate::functions::swppv::stf::STF_RR_SWPPV::STF_RR_SWPPV;
use crate::functions::swppv::stf::STF_TD_SWPPV::STF_TD_SWPPV;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

pub struct SWPPV;

impl TraitContractModel for SWPPV {
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_FXOUT)),
                Some(Rc::new(STF_PRD_SWPPV)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Initial exchange event
        let e: ContractEvent<InitialExchangeDate, InitialExchangeDate> = EventFactory::create_event(
            &model.initial_exchange_date.clone(),
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_SWPPV)),
            Some(Rc::new(STF_IED_SWPPV)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption event
        let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &model.maturity_date.clone().map(|rc| (*rc).clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_SWPPV)),
            Some(Rc::new(STF_MD_SWPPV)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Interest payment events
        if model.delivery_settlement == Some(DeliverySettlement::D(D)) || model.delivery_settlement.is_none() {
            // In case of physical delivery (delivery of individual cash flows)
            let interest_schedule = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_interest_payment.clone(),
                &model.end_of_month_convention.clone(),
                Some(false)
            );

            // Fixed rate events
            let fixed_rate_events = EventFactory::create_events(
                &interest_schedule,
                &EventType::IPFX,
                &model.currency,
                Some(Rc::new(POF_IPFix_SWPPV)),
                Some(Rc::new(STF_IPFix_SWPPV)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            // Floating rate events
            let floating_rate_events = EventFactory::create_events(
                &interest_schedule,
                &EventType::IPFL,
                &model.currency,
                Some(Rc::new(POF_IPFloat_SWPPV)),
                Some(Rc::new(STF_IPFloat_SWPPV)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fixed_rate_events);
            events.extend(floating_rate_events);
        } else {
            // In case of cash delivery (cash settlement)
            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_payment.clone(),
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(true)
                ),
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_SWPPV)),
                Some(Rc::new(STF_IP_SWPPV)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_rate_reset,
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_rate_reset,
                &model.end_of_month_convention,
                Some(false),
            ),
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_SWPPV)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        events.extend(rate_reset_events);

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_FXOUT)),
                Some(Rc::new(STF_TD_SWPPV)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            &model.status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= to_event.event_time);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        Ok(events)
    }

    fn apply(
        events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &DataObserver,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let _maturity = &model.maturity_date.clone();
        let mut states = Self::init_state_space(model, observer, _maturity).expect("Failed to initialize state_space");
        let mut events = events.clone();

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.clone(),
                model.business_day_adjuster.as_ref().unwrap(),
            );
        }

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &model.purchase_date {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        Ok(events)
    }

    fn init_state_space(model: &ContractModel, _observer: &DataObserver, _maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        states.status_date = model.status_date.clone();

        if model.initial_exchange_date.clone().unwrap().value() <= model.status_date.clone().unwrap().value() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
            states.nominal_interest_rate2 = model.nominal_interest_rate2.clone();
            states.accrued_interest = AccruedInterest::new(role_sign * model.accrued_interest.clone().unwrap().value()).ok();
            states.accrued_interest2 = AccruedInterest2::new(role_sign * model.accrued_interest2.clone().unwrap().value()).ok();
            states.last_interest_period = Some(0.0);
        }

        Ok(states)
    }
}
impl fmt::Display for SWPPV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMPPV")
    }
}