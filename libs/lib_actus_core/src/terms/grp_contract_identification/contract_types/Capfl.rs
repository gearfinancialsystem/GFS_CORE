
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
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;

pub struct CAPFL;

impl TraitContractModel for CAPFL {
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        // Compute underlying event schedule
        let underlying_model = model.contract_structure.clone().unwrap().0
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| Some(c.object.clone().as_cm()))
            .map(|obj| {
                let mut m = obj.unwrap();
                m.contract_role = Some(ContractRole::new(Some("RPA")).expect("good contract role")); //   .add_attribute("contractRole", ContractRole::RPA);
                m
            })
            .ok_or("Underlying model not found")?;

        let umat = underlying_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();
        let mut events = ContractType::schedule(
            Some(umat),
            &underlying_model,
        ).unwrap();

        // Purchase
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_STK)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Termination
        if let Some(termination_date) = &model.termination_date {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_STK)),
                Some(Rc::new(STF_TD_STK)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.compare_to(&termination.to_iso_datetime_event()) != 1);
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&status_event.to_iso_datetime_event()) != -1);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&to_event) != 1);

        Ok(events)
    }

    fn apply(
        mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        // Evaluate events of underlying without cap/floor applied
        let underlying_model = model.contract_structure.clone().unwrap().0
            .iter()
            .find(|c| c.reference_role == ReferenceRole::UDL)
            .and_then(|c| Some(c.object.clone()))
            .ok_or("Underlying model not found").unwrap();

        let underlying_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = ContractType::apply(events.clone(), &underlying_model.as_cm().unwrap(), observer).unwrap()
            .into_iter()
            .filter(|e| e.event_type == EventType::IP)
            .collect();//::<Vec<_>>();

        // Evaluate events of underlying with cap/floor applied
        let mut underlying_model_with_cap_floor = underlying_model.clone().as_cm().unwrap();
        underlying_model_with_cap_floor.life_cap = model.life_cap.clone();
        underlying_model_with_cap_floor.life_floor = model.life_floor.clone();

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
            let purchase_event: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.compare_to(&purchase_event.to_iso_datetime_event()) != -1);
        }

        Ok(events)
    }

    fn init_state_space(model: &ContractModel, _observer: &RiskFactorModel, _maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String> {
        todo!()
    }
}

impl CAPFL {
    pub fn netting_event(
        e1: &ContractEvent<IsoDatetime, IsoDatetime>,
        e2: &ContractEvent<IsoDatetime, IsoDatetime>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> ContractEvent<IsoDatetime, IsoDatetime> {
        let mut e = EventFactory::create_event(
            &e1.event_time,
            &e1.event_type,
            &e1.currency,
            Some(Rc::new(POF_NET_CAPFL::new(e1.clone(), e2.clone()))),
            Some(Rc::new(STF_NET_CAPFL::new(e1.clone(), e2.clone()))),
            &None,
            &model.contract_id,
        );

        e.eval(
            &mut StateSpace::default(),
            model,
            observer,
            &DayCountConvention::new(Some("AAISDA"), None, None).ok(),//&DayCountCalculator::new("AA", Box::new(NoHolidaysCalendar)),
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