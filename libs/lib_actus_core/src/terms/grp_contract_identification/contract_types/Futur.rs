
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
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

pub struct FUTUR;

impl FUTUR {
    pub fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_OPTNS)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Exercise and Settlement events
        if let Some(exercise_date) = &model.exercise_date {
            let e : ContractEvent<ExerciseDate, ExerciseDate> = EventFactory::create_event(
                &Some(exercise_date.clone()),
                &EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_FUTUR)),
                Some(Rc::new(STF_XD_FUTUR)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(exercise_date.clone().value() + model.settlement_period.clone().unwrap().value().clone())
            );

            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                &None,
                &model.contract_id,
            );

            events.push(e.to_iso_datetime_event());
        } else {
            let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &EventType::XD,
                &model.currency,
                Some(Rc::new(POF_XD_FUTUR)),
                Some(Rc::new(STF_XD_FUTUR)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());

            let settlement_date = model.business_day_adjuster.as_ref().unwrap().shift_bd(
                &(model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().clone().value() + model.settlement_period.clone().unwrap().value().clone())
            );
            let e: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
                &Some(settlement_date),
                &EventType::STD,
                &model.currency,
                Some(Rc::new(POF_STD_OPTNS)),
                Some(Rc::new(STF_STD_OPTNS)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Maturity event
        let e : ContractEvent<MaturityDate, MaturityDate>= EventFactory::create_event(
            &model.maturity_date.clone().map(|rc| (*rc).clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_FUTUR)),
            Some(Rc::new(STF_MD_FUTUR)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_OPTNS)),
                Some(Rc::new(STF_TD_STK)),
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

        Ok(events)
    }

    pub fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>, model: &ContractModel, observer: &RiskFactorModel)
        -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        let mut states = Self::init_state_space(model).expect("Failed to initialize state space.");
        let mut events = events.clone();

        // Add external XD-event
        events.extend(observer.events(model));

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

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
            let purchase_event: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e >= &purchase_event.to_iso_datetime_event());
        }

        Ok(events)
    }

    fn init_state_space(model: &ContractModel) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();

        states.status_date = model.status_date.clone();
        states.exercise_amount = model.exercise_amount.clone();
        states.exercise_date = model.exercise_date.clone();
        states.contract_performance = model.contract_performance.clone();

        Ok(states)
    }
}
impl fmt::Display for FUTUR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FUTUR")
    }
}