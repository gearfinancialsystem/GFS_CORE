use std::error::Error;
use std::rc::Rc;
use std::collections::HashMap;

use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;

use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::functions::capfl::pof::POF_NET_CAPFL::POF_NET_CAPFL;
use crate::functions::capfl::stf::STF_NET_CAPFL::STF_NET_CAPFL;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::types::isoDatetime::IsoDatetime;

pub struct CapFloor;

impl CapFloor {
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        // Compute underlying event schedule
        let underlying_model = model.contractStructure
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| c.object.clone())
            .map(|obj| {
                let mut m = obj;
                m.add_attribute("contractRole", ContractRole::RPA);
                m
            })
            .ok_or("Underlying model not found")?;

        let mut events = ContractType::schedule(
            underlying_model.maturity_date.as_ref().unwrap(),
            &underlying_model,
        )?;

        // Purchase
        if let Some(purchase_date) = &model.purchaseDate {
            events.push(EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                Some(Rc::new(POF_PRD_STK)),
                Some(Rc::new(STF_PRD_STK)),
                model.contractID.as_ref(),
            ));
        }

        // Termination
        if let Some(termination_date) = &model.terminationDate {
            let termination = EventFactory::create_event(
                Some(termination_date.clone()),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_STK)),
                Some(Rc::new(STF_TD_STK)),
                model.contractID.as_ref(),
            );

            events.retain(|e| e.compare_to(&termination) != 1);
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

        events.retain(|e| e.compare_to(&status_event) != -1);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            model.currency.as_ref(),
            None,
            None,
            model.contractID.as_ref(),
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
        let underlying_model = model.contractStructure.clone().unwrap()
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| c.object.clone())
            .ok_or("Underlying model not found")?;

        let mut underlying_events = ContractType::apply(events.clone(), &underlying_model, observer)?
            .into_iter()
            .filter(|e| e.eventType == EventType::IP)
            .collect::<Vec<_>>();

        // Evaluate events of underlying with cap/floor applied
        let mut underlying_model_with_cap_floor = underlying_model.clone();
        underlying_model_with_cap_floor.add_attribute("lifeCap", model.lifeCap);
        underlying_model_with_cap_floor.add_attribute("lifeFloor", model.lifeFloor);

        let mut underlying_with_cap_floor_events = events
            .into_iter()
            .map(|e| e.copy())
            .collect::<Vec<_>>();

        underlying_with_cap_floor_events = ContractType::apply(underlying_with_cap_floor_events, &underlying_model_with_cap_floor, observer)?
            .into_iter()
            .filter(|e| e.eventType == EventType::IP)
            .collect::<Vec<_>>();

        // Net schedules of underlying with and without cap/floor applied
        let mut merged_events = HashMap::new();

        for e in underlying_events.iter().chain(underlying_with_cap_floor_events.iter()) {
            let key = format!("{:?}{:?}", e.eventTime, e.eventType);
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
        if let Some(purchase_date) = &model.purchaseDate {
            let purchase_event = EventFactory::create_event(
                Some(purchase_date.clone()),
                EventType::PRD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref(),
            );

            events.retain(|e| e.eventType == EventType::AD || e.compare_to(&purchase_event) != -1);
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
            e1.eventTime.clone(),
            e1.eventType,
            e1.currency.as_ref(),
            Some(Rc::new(POF_NET_CAPFL::new(e1, e2))),
            Some(Rc::new(STF_NET_CAPFL::new(e1, e2))),
            model.contractID.as_ref(),
        );

        e.eval(
            &mut StateSpace::default(),
            model,
            observer,
            &DayCountCalculator::new("AA", Box::new(NoHolidaysCalendar)),
            model.businessDayAdjuster.as_ref().unwrap(),
        );

        e
    }
}
