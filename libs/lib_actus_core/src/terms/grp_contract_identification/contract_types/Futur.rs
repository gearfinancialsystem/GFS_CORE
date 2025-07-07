use std::error::Error;
use std::fmt;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;

use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractModel::ContractModel;
use crate::functions::futur::pof::POF_MD_FUTUR::POF_MD_FUTUR;
use crate::functions::futur::pof::POF_XD_FUTUR::POF_XD_FUTUR;
use crate::functions::futur::stf::STF_MD_FUTUR::STF_MD_FUTUR;
use crate::functions::futur::stf::STF_XD_FUTUR::STF_XD_FUTUR;
use crate::functions::optns::pof::POF_PRD_OPTNS::POF_PRD_OPTNS;
use crate::functions::optns::pof::POF_STD_OPTNS::POF_STD_OPTNS;
use crate::functions::optns::pof::POF_TD_OPTNS::POF_TD_OPTNS;
use crate::functions::optns::stf::STF_STD_OPTNS::STF_STD_OPTNS;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;

pub struct FUTUR;

impl FUTUR {
    pub fn schedule(to: &IsoDatetime, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
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
                Some(Rc::new(POF_XD_FUTUR)),
                Some(Rc::new(STF_XD_FUTUR)),
                &model.contract_id,
            ));

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(exercise_date.clone() + model.settlement_period.clone().unwrap())
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
                Some(Rc::new(POF_XD_FUTUR)),
                Some(Rc::new(STF_XD_FUTUR)),
                &model.contract_id,
            ));

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().clone() + model.settlement_period.clone().unwrap())
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
            Some(Rc::new(POF_MD_FUTUR)),
            Some(Rc::new(STF_MD_FUTUR)),
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

            events.retain(|e| e <= &termination);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            model.status_date,
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e >= &status_event);

        Ok(events)
    }

    pub fn apply(events: Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel) -> Vec<ContractEvent> {
        let mut states = Self::init_state_space(model);
        let mut events = events.clone();

        // Add external XD-event
        events.extend(observer.events(model));

        events.sort();

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.as_ref().unwrap(),
                &model.business_day_adjuster.as_ref().unwrap(),
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

            events.retain(|e| e.event_type == EventType::AD || e >= &purchase_event);
        }

        events
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.status_date = model.status_date;
        states.exercise_amount = model.exerciseAmount;
        states.exercise_date = model.exercise_date;
        states.contract_performance = model.contract_performance;

        states
    }
}
impl fmt::Display for FUTUR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FUTUR")
    }
}