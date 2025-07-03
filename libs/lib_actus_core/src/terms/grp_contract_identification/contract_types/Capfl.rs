use std::error::Error;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::functions::capfl::pof::POF_NET_CAPFL::POF_NET_CAPFL;
use crate::functions::capfl::stf::STF_NET_CAPFL::STF_NET_CAPFL;
use crate::functions::stk::pof::POF_PRD_STK::POF_PRD_STK;
use crate::functions::stk::pof::POF_TD_STK::POF_TD_STK;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::types::IsoDatetime::IsoDatetime;

pub struct CAPFL;

impl CAPFL {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        // Compute underlying event schedule
        let underlying_model = model.contract_structure.clone().unwrap()
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| Some(c.object.clone().as_cm()))
            .map(|obj| {
                let mut m = obj.unwrap();
                m.contractRole = Some(ContractRole::new(Some("RPA")).expect("good contract role")); //   .add_attribute("contractRole", ContractRole::RPA);
                m
            })
            .ok_or("Underlying model not found")?;

        let mut events = ContractType::schedule(
            underlying_model.maturity_date.clone().map(|rc| (*rc).clone()),
            &underlying_model,
        ).unwrap();

        // Purchase
        if let Some(purchase_date) = &model.purchase_date {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_STK)),
                Some(Rc::new(STF_PRD_STK)),
                &model.contract_id,
            ));
        }

        // Termination
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_STK)),
                Some(Rc::new(STF_TD_STK)),
                &model.contract_id,
            );

            events.retain(|e| e.compare_to(&termination) != 1);
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

        events.retain(|e| e.compare_to(&status_event) != -1);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&to_event) != 1);

        Ok(events)
    }

    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        // Evaluate events of underlying without cap/floor applied
        let underlying_model = model.contract_structure.clone().unwrap()
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| Some(c.object.clone()))
            .ok_or("Underlying model not found").unwrap();

        let underlying_events: Vec<ContractEvent> = ContractType::apply(events.clone(), &underlying_model.as_cm().unwrap(), observer).unwrap()
            .into_iter()
            .filter(|e| e.event_type == EventType::IP)
            .collect();//::<Vec<_>>();

        // Evaluate events of underlying with cap/floor applied
        let mut underlying_model_with_cap_floor = underlying_model.clone().as_cm().unwrap();
        underlying_model_with_cap_floor.lifeCap = &model.life_cap;
        underlying_model_with_cap_floor.lifeFloor = &model.life_floor;

        let mut underlying_with_cap_floor_events = events
            .into_iter()
            .map(|e| e.copy())
            .collect();

        underlying_with_cap_floor_events = ContractType::apply(underlying_with_cap_floor_events, &underlying_model_with_cap_floor, observer).unwrap()
            .into_iter()
            .filter(|e| e.event_type == EventType::IP)
            .collect::<Vec<_>>();

        // Net schedules of underlying with and without cap/floor applied
        let mut merged_events = HashMap::new();

        for e in underlying_events.iter().chain(underlying_with_cap_floor_events.iter()) {
            let key = format!("{:?}{:?}", e.event_time, e.event_type);
            let existing_event = merged_events.get(&key);

            let new_event = match existing_event {
                Some(existing) => Self::netting_event(existing, e, model, observer),
                None => e.copy(),
            };

            merged_events.insert(key, new_event);
        }

        let mut events = Vec::from_iter(merged_events.into_values());
        events.sort();

        // Remove pre-purchase events if purchase date set
        if let Some(purchase_date) = &model.purchase_date {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.compare_to(&purchase_event) != -1);
        }

        events
    }

    pub fn netting_event(
        e1: &ContractEvent,
        e2: &ContractEvent,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> ContractEvent {
        let mut e = EventFactory::create_event(
            &e1.event_time,
            &e1.event_type,
            &e1.currency,
            Some(Rc::new(POF_NET_CAPFL::new(e1.clone(), e2.clone()))),
            Some(Rc::new(STF_NET_CAPFL::new(e1.clone(), e2.clone()))),
            &model.contract_id,
        );

        e.eval(
            &mut StateSpace::default(),
            model,
            observer,
            &DayCountConvention::new(Some("AAISDA"), None, None).expect("dfe"),//&DayCountCalculator::new("AA", Box::new(NoHolidaysCalendar)),
            model.business_day_adjuster.as_ref().unwrap(),
        );

        e
    }
}
impl fmt::Display for CAPFL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CAPFL")
    }
}