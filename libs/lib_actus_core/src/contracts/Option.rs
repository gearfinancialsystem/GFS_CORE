use std::error::Error;
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
use crate::types::isoDatetime::IsoDatetime;
use crate::util::CycleUtils;

pub struct Option;

impl Option {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchaseDate {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_STK)),
                model.contractID.as_ref(),
            ));
        }

        // Exercise and Settlement events
        if let Some(exercise_date) = &model.exerciseDate {
            events.push(EventFactory::create_event(
                Some(exercise_date.clone()),
                EventType::XD,
                model.currency.as_ref(),
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_OPTNS)),
                model.contractID.as_ref(),
            ));

            let settlement_date = model.businessDayAdjuster.as_ref().unwrap().shift_bd(
                &(*exercise_date + model.clone().settlementPeriod.unwrap())
            );

            events.push(EventFactory::create_event(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                model.contractID.as_ref(),
            ));
        } else {
            events.push(EventFactory::create_event(
                model.maturityDate.clone().map(|rc| (*rc).clone()),
                EventType::XD,
                model.currency.as_ref(),
                Some(Rc::new(POF_XD_OPTNS)),
                Some(Rc::new(STF_XD_OPTNS)),
                model.contractID.as_ref(),
            ));

            let settlement_date = model.businessDayAdjuster.as_ref().unwrap().shift_bd(
                &(model.maturityDate.clone().map(|rc| (*rc).clone()).unwrap() + model.settlementPeriod.clone().unwrap())
            );

            events.push(EventFactory::create_event(
                Some(settlement_date),
                EventType::STD,
                model.currency.as_ref(),
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                model.contractID.as_ref(),
            ));
        }

        // Maturity event
        events.push(EventFactory::create_event(
            model.maturityDate.clone().map(|rc| (*rc).clone()),
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_OPTNS)),
            Some(Rc::new(STF_MD_OPTNS)),
            model.contractID.as_ref(),
        ));

        // Termination event
        if let Some(termination_date) = &model.terminationDate {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_OPTNS)),
                Some(Rc::new(STF_TD_STK)),
                model.contractID.as_ref(),
            );

            events.retain(|e| e.eventTime <= termination.eventTime);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            model.statusDate.clone(),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
        );

        events.retain(|e| e.eventTime >= status_event.eventTime);

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

        events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                model.dayCountConvention.as_ref().unwrap(),
                model.businessDayAdjuster.as_ref().unwrap(),
            );
        }

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &model.purchaseDate {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );

            events.retain(|e| e.eventType == EventType::AD || e.eventTime >= purchase_event.eventTime);
        }

        events
    }

    fn init_state_space(model: &ContractModel) -> StateSpace {
        let mut states = StateSpace::default();

        states.statusDate = model.statusDate;
        states.exerciseAmount = model.exerciseAmount;
        states.exerciseDate = model.exerciseDate;
        states.contractPerformance = model.contractPerformance;

        states
    }
}
