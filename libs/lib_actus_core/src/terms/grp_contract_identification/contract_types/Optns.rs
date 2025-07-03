use std::error::Error;
use std::fmt;
use std::rc::Rc;

use crate::attributes::ContractModel::ContractModel;
use crate::events::{ContractEvent::ContractEvent, EventFactory::EventFactory, EventType::EventType};
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::functions::optns::pof::POF_MD_OPTNS::POF_MD_OPTNS;
use crate::functions::optns::pof::POF_PRD_OPTNS::POF_PRD_OPTNS;
use crate::functions::optns::pof::POF_STD_OPTNS::POF_STD_OPTNS;
use crate::functions::optns::pof::POF_TD_OPTNS::POF_TD_OPTNS;
use crate::functions::optns::pof::POF_XD_OPTNS::POF_XD_OPTNS;
use crate::functions::optns::stf::STF_MD_OPTNS::STF_MD_OPTNS;
use crate::functions::optns::stf::STF_STD_OPTNS::STF_STD_OPTNS;
use crate::functions::optns::stf::STF_XD_OPTNS::STF_XD_OPTNS;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::types::IsoDatetime::IsoDatetime;

pub struct OPTNS;

impl OPTNS {
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
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_STK)),
                &model.contract_id,
            ));
        }

        // Exercise and Settlement events
        if let Some(exercise_date) = &model.exercise_date {
            events.push(EventFactory::create_event(
                Some(exercise_date.clone()),
                EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_OPTNS)),
                &model.contract_id,
            ));

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(*exercise_date + model.clone().settlementPeriod.unwrap())
            );

            events.push(EventFactory::create_event(
                Some(settlement_date),
                EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                &model.contract_id,
            ));
        } else {
            events.push(EventFactory::create_event(
                model.maturity_date.clone().map(|rc| (*rc).clone()),
                EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_OPTNS)),
                &model.contract_id,
            ));

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap() + model.settlement_period.clone().unwrap())
            );

            events.push(EventFactory::create_event(
                Some(settlement_date),
                EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                &model.contract_id,
            ));
        }

        // Maturity event
        events.push(EventFactory::create_event(
            model.maturity_date.clone().map(|rc| (*rc).clone()),
            EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_OPTNS)),
            Some(Rc::new(STF_MD_OPTNS)),
            &model.contract_id,
        ));

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_OPTNS)),
                Some(Rc::new(STF_TD_STK)),
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

        Ok(events)
    }

    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events.clone();

        // Add external XD-event
        events.extend(observer.events(model));

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

        states.status_date = model.status_date;
        states.exercise_amount = model.exerciseAmount;
        states.exerciseDate = model.exercise_date;
        states.contract_performance = model.contract_performance;

        states
    }
}
impl fmt::Display for OPTNS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OPTNS")
    }
}