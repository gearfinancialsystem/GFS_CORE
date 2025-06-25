use std::error::Error;
use std::rc::Rc;
use crate::events::ContractEvent;
use crate::events::EventFactory;
use crate::events::EventType;
use crate::externals::RiskFactorModel;
use crate::functions::futur::{POF_MD_FUTUR, POF_XD_FUTUR, STF_MD_FUTUR, STF_XD_FUTUR};
use crate::functions::optns::{POF_PRD_OPTNS, POF_STD_OPTNS, POF_TD_OPTNS, STF_STD_OPTNS};
use crate::functions::stk::{STF_PRD_STK, STF_TD_STK};
use crate::state_space::StateSpace;
use crate::types::isoDatetime::IsoDatetime;
use crate::attributes::ContractModel;
use crate::conventions::businessday::BusinessDayAdjuster;

pub struct Future;

impl Future {
    pub fn schedule(to: IsoDatetime, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
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
                Some(Rc::new(POF_XD_FUTUR)),
                Some(Rc::new(STF_XD_FUTUR)),
                model.contractID.as_ref(),
            ));

            let settlement_date = model.businessDayAdjuster.as_ref().unwrap().shift_event_time(
                exercise_date.plus_period(&model.settlementPeriod.unwrap())
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
                model.maturityDate,
                EventType::XD,
                model.currency.as_ref(),
                Some(Rc::new(POF_XD_FUTUR)),
                Some(Rc::new(STF_XD_FUTUR)),
                model.contractID.as_ref(),
            ));

            let settlement_date = model.businessDayAdjuster.as_ref().unwrap().shift_event_time(
                model.maturityDate.unwrap().plus_period(&model.settlementPeriod.unwrap())
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
            model.maturityDate,
            EventType::MD,
            model.currency.as_ref(),
            Some(Rc::new(POF_MD_FUTUR)),
            Some(Rc::new(STF_MD_FUTUR)),
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

            events.retain(|e| e <= &termination);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            model.statusDate,
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
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
                &model.dayCountConvention.as_ref().unwrap(),
                &model.businessDayAdjuster.as_ref().unwrap(),
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

            events.retain(|e| e.eventType == EventType::AD || e >= &purchase_event);
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
