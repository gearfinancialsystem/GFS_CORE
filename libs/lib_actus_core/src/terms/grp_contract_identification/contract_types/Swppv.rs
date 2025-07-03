use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::attributes::ContractModel::ContractModel;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::externals::RiskFactorModel::RiskFactorModel;
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
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::types::IsoDatetime::IsoDatetime;

pub struct SWPPV;

impl SWPPV {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_FXOUT)),
                Some(Rc::new(STF_PRD_SWPPV)),
                &model.contract_id,
            ));
        }

        // Initial exchange event
        events.push(EventFactory::create_event(
            model.initial_exchange_date.clone(),
            EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_SWPPV)),
            Some(Rc::new(STF_IED_SWPPV)),
            &model.contract_id,
        ));

        // Principal redemption event
        events.push(EventFactory::create_event(
            model.maturity_date.clone().map(|rc| (*rc).clone()),
            EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_SWPPV)),
            Some(Rc::new(STF_MD_SWPPV)),
            &model.contract_id,
        ));

        // Interest payment events
        if model.deliverySettlement == Some(DeliverySettlement::D(D)) || model.deliverySettlement.is_none() {
            // In case of physical delivery (delivery of individual cash flows)
            let interest_schedule = ScheduleFactory::create_schedule_end_time_true(
                model.cycle_anchor_date_of_Interest_payment.clone(),
                model.maturity_date.clone().map(|rc| (*rc).clone()),
                model.cycle_of_interest_payment.clone(),
                model.end_of_month_convention.clone().unwrap(),
            );

            // Fixed rate events
            let fixed_rate_events = EventFactory::create_events_with_convention(
                &interest_schedule,
                EventType::IPFX,
                &model.currency,
                Some(Rc::new(POF_IPFix_SWPPV)),
                Some(Rc::new(STF_IPFix_SWPPV)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            // Floating rate events
            let floating_rate_events = EventFactory::create_events_with_convention(
                &interest_schedule,
                EventType::IPFL,
                &model.currency,
                Some(Rc::new(POF_IPFloat_SWPPV)),
                Some(Rc::new(STF_IPFloat_SWPPV)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            events.extend(fixed_rate_events);
            events.extend(floating_rate_events);
        } else {
            // In case of cash delivery (cash settlement)
            let interest_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule_end_time_true(
                    model.cycle_anchor_date_of_Interest_payment.clone(),
                    model.maturity_date.clone().map(|rc| (*rc).clone()),
                    model.cycle_of_interest_payment.clone(),
                    model.end_of_month_convention.clone().unwrap(),
                ),
                EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_SWPPV)),
                Some(Rc::new(STF_IP_SWPPV)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycle_anchor_date_of_rate_reset.clone(),
                model.maturity_date.clone().map(|rc| (*rc).clone()),
                model.cycle_of_rate_reset.clone(),
                model.end_of_month_convention.clone().unwrap(),
                false,
            ),
            EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_SWPPV)),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        );

        events.extend(rate_reset_events);

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_FXOUT)),
                Some(Rc::new(STF_TD_SWPPV)),
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            model.status_date.clone(),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= to_event.event_time);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        Ok(events)
    }

    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events.clone();

        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                model.day_count_convention.as_ref().unwrap(),
                model.business_day_adjuster.as_ref().unwrap(),
            );
        }

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &model.purchase_date {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        events
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = Some(1.0);
        states.status_date = model.status_date;

        if model.initial_exchange_date <= model.status_date {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = Some(role_sign * model.notional_principal.unwrap());
            states.nominal_interest_rate = model.nominal_interest_rate;
            states.nominal_interest_rate2 = model.nominal_interest_rate2;
            states.accrued_interest = Some(role_sign * model.accrued_interest.unwrap());
            states.accrued_interest2 = Some(role_sign * model.accrued_interest2.unwrap());
            states.lastInterestPeriod = Some(0.0);
        }

        states
    }
}
impl fmt::Display for SWPPV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMPPV")
    }
}