use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::attributes::ContractModel::ContractModel;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::functions::clm::pof::POF_IED_CLM::POF_IED_CLM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::pof::POF_TD_PAM::POF_TD_PAM;
use crate::functions::pam::stf::STF_FP_PAM::STF_FP_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::pam::stf::STF_IPCI_PAM::STF_IPCI_PAM;
use crate::functions::pam::stf::STF_RR_PAM::STF_RR_PAM;
use crate::functions::pam::stf::STF_RRF_PAM::STF_RRF_PAM;
use crate::functions::pam::stf::STF_TD_PAM::STF_TD_PAM;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::types::IsoDatetime::IsoDatetime;

pub struct UMP;

impl UMP {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();

        // Initial exchange event
        events.push(EventFactory::create_event(
            model.initial_exchange_date.clone(),
            EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_CLM)),
            Some(Rc::new(STF_IED_PAM)),
            &model.contract_id,
        ));

        // Interest payment capitalization events
        let interest_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycle_anchor_date_of_Interest_payment.clone(),
                Some(to.clone()),
                model.cycle_of_interest_payment.clone(),
                model.end_of_month_convention.clone().unwrap(),
                false,
            ),
            EventType::IPCI,
            &model.currency,
            Some(Rc::new(POF_IPCI_PAM)),
            Some(Rc::new(STF_IPCI_PAM)),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        );

        events.extend(interest_events);

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                model.cycle_anchor_date_of_rate_reset.clone(),
                Some(to.clone()),
                model.cycle_of_rate_reset.clone(),
                model.end_of_month_convention.clone().unwrap(),
                false,
            ),
            EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            model.business_day_adjuster.as_ref().unwrap(),
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::create_event(
                model.status_date.clone(),
                EventType::AD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

            let mut fixed_event = sorted_events.iter().find(|&e| e.event_time.clone() > status_event.event_time.clone()).unwrap().clone().clone();
            fixed_event.fstate = Some(Rc::new(STF_RRF_PAM));
            fixed_event.event_type = EventType::RRF;
            rate_reset_events.insert(fixed_event);

        }

        events.append(&mut rate_reset_events.into_iter().collect());

        // Fee events (if specified)
        if let Some(cycle_of_fee) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_fee.clone(),
                    Some(to.clone()),
                    Some(cycle_of_fee.clone()),
                    model.end_of_month_convention.clone().unwrap(),
                    false,
                ),
                EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_PAM)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_PAM)),
                Some(Rc::new(STF_TD_PAM)),
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

        events
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = Some(1.0);
        states.interest_scaling_multiplier = Some(1.0);
        states.status_date = model.status_date;

        if model.initial_exchange_date <= model.status_date {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = Some(role_sign * model.notional_principal.unwrap());
            states.nominal_interest_rate = model.nominal_interest_rate;
            states.accrued_interest = Some(role_sign * model.accrued_interest.unwrap());
            states.fee_accrued = model.fee_accrued;
        }

        states
    }
}
impl fmt::Display for UMP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UMP")
    }
}