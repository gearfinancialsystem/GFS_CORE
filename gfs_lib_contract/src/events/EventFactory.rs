
use std::collections::HashSet;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;


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
        let dd = if convention.is_none() {
            EventTime::new(schedule_time.unwrap().value()).ok()
        }
        else {
            let time = schedule_time.as_ref().unwrap();
            let tmp_time: PhantomIsoDatetimeW = time.convert();
            let adjusted_time = convention.clone().unwrap().shift_bd(&tmp_time);
            let conventionx = Some(adjusted_time);
            EventTime::new(conventionx.clone().unwrap().value()).ok()
        };

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
                let dd = if convention.is_none() {
                    EventTime::new(time.clone().value()).ok()
                }
                else {
                    let tmp_time: PhantomIsoDatetimeW = time.convert();
                    let adjusted_time = convention.clone().unwrap().shift_bd(&tmp_time);
                    EventTime::new(adjusted_time.clone().value()).ok()
                };

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

