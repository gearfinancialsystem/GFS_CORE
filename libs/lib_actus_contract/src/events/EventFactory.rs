
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::marker::PhantomData;
use lib_actus_terms::non_terms::EventTime::EventTime;
use lib_actus_terms::non_terms::ScheduleTime::ScheduleTime;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
//use crate::events::AnyContractEvent::AnyContractEvent;

use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

pub struct EventFactory {
}

impl EventFactory {
    // Créez une nouvelle instance de EventFactory
    pub fn new() -> Self {
        EventFactory {
        }
    }
    pub fn create_event(
        schedule_time: &Option<ScheduleTime>, // old T1
        event_type: &EventType,
        currency: &Option<Currency>,
        pay_off: Option<PayOffFunction>,
        state_trans: Option<StatesTransitionFunction>,
        convention: &Option<BusinessDayAdjuster>,
        contract_id: &Option<ContractID>
    ) -> ContractEvent
    {
        let mut dd = None;

        if convention.is_none() {
            //let schedule_time_copy2 = PhantomIsoDatetimeW::new(schedule_time_copy) ;
            dd = EventTime::new(schedule_time.unwrap().value()).ok();
        }
        else {
            let time = schedule_time.as_ref().unwrap();
            let adjusted_time = convention.clone().unwrap().shift_bd(&time.to_phantom_type());
            let conventionx = Some(adjusted_time);
            dd = EventTime::new(conventionx.clone().unwrap().value()).ok()
        }

        ContractEvent::new(
            schedule_time,
            &dd,
            event_type,
            currency,
            pay_off,
            state_trans,
            contract_id,
        )
    }

    /// Create a series of `ContractEvent`s from an unordered schedule of times
    pub fn create_events(
        event_schedule: &HashSet<ScheduleTime>,
        event_type: &EventType,
        currency: &Option<Currency>,
        pay_off: Option<PayOffFunction>,
        state_trans: Option<StatesTransitionFunction>,
        convention: &Option<BusinessDayAdjuster>,
        contract_id: &Option<ContractID>,
    ) -> HashSet<ContractEvent> {
        
        event_schedule
            .iter()
            .map(|time| {
                let mut dd = None;
                if convention.is_none() {

                    dd = EventTime::new(time.clone().value()).ok() ;
                    
                }
                else {

                    let adjusted_time = convention.clone().unwrap().shift_bd(&time.to_phantom_type());
                    dd = EventTime::new(adjusted_time.clone().value()).ok()
                }

                ContractEvent::new(
                    &Some(time.clone()),
                    &dd,
                    event_type,
                    currency,
                    pay_off.clone(),
                    state_trans.clone(),
                    contract_id,
                )
            }).collect()
    }
}

