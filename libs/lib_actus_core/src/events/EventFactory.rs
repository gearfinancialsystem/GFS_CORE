use std::collections::HashSet;
use std::rc::Rc;
use chrono::DurationRound;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

pub struct EventFactory;

impl EventFactory {

    /// Create a single `ContractEvent`
    pub fn create_event(
        schedule_time: Option<IsoDatetime>,
        event_type: EventType,
        currency: Option<String>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        contract_id: Option<String>,
    ) -> ContractEvent {
        ContractEvent::new(
            schedule_time,
            schedule_time,
            event_type,
            currency,
            pay_off,
            state_trans,
            contract_id,
        )
    }


    /// Create a single `ContractEvent` with adjusted event time based on a business day convention
    pub fn create_event_with_convention(
        schedule_time: Option<IsoDatetime>,
        event_type: EventType,
        currency: Option<String>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        convention: &BusinessDayConvention,
        contract_id: Option<String>,
    ) -> ContractEvent {
        let adjusted_time = Some(convention.shift_bd(&schedule_time.unwrap()));
        ContractEvent::new(
            schedule_time,
            Some(convention.shift_bd(&schedule_time.unwrap())),
            event_type,
            currency,
            pay_off,
            state_trans,
            contract_id,
        )

    }

    /// Create a series of `ContractEvent`s from an unordered schedule of times
    pub fn create_events(
        event_schedule: &HashSet<IsoDatetime>,
        event_type: EventType,
        currency: Option<String>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        contract_id: Option<String>,
    ) -> HashSet<ContractEvent> {
        event_schedule
            .iter()
            .map(|&time| {
                ContractEvent::new(
                    Some(time),
                    Some(time),
                    event_type.clone(),
                    currency.clone(),
                    pay_off.clone(),
                    state_trans.clone(),
                    contract_id.clone(),
                )
            }).collect()
    }

    /// Create a series of `ContractEvent`s from an unordered schedule of times with business day convention adjustments
    ///
    /// # Parameters
    /// - `event_schedule`: An unordered set of schedule times
    /// - `event_type`: The cont_type of the event
    /// - `currency`: The currency associated with the events
    /// - `pay_off`: The pay-off function associated with the events
    /// - `state_trans`: The state-transition function associated with the events
    /// - `convention`: The business day convention to apply for time adjustments
    /// - `contract_id`: The ID of the contract
    pub fn create_events_with_convention(
        event_schedule: &HashSet<IsoDatetime>,
        event_type: EventType,
        currency: Option<String>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        convention: &BusinessDayConvention,
        contract_id: Option<String>,
    ) -> HashSet<ContractEvent> {
        event_schedule
            .iter()
            .map(|&time| {
                let adjusted_time = convention.shift_bd(&time);
                ContractEvent::new(
                    Some(time),
                    Some(convention.shift_bd(&time)),
                    event_type.clone(),
                    currency.clone(),
                    pay_off.clone(),
                    state_trans.clone(),
                    contract_id.clone(),
                )
            })
            .collect()
    }
}
