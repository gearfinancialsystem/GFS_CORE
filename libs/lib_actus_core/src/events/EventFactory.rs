use std::collections::HashSet;
use std::rc::Rc;
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
        pay_off: Rc<dyn TraitPayOffFunction>,
        state_trans: Rc<dyn TraitStateTransitionFunction>,
        contract_id: Option<String>,
    ) -> ContractEvent {
        ContractEvent {
            contractID: contract_id,
            currency: currency,
            eventTime: schedule_time,
            eventType: event_type,
            payoff: pay_off,
            scheduleTime: schedule_time,
            state: state_trans
        }
        
    }


    /// Create a single `ContractEvent` with adjusted event time based on a business day convention
    pub fn create_event_with_convention(
        schedule_time: Option<IsoDatetime>,
        event_type: EventType,
        currency: Option<String>,
        pay_off: Rc<dyn TraitPayOffFunction>,
        state_trans: Rc<dyn TraitStateTransitionFunction>,
        convention: &BusinessDayConvention,
        contract_id: Option<String>,
    ) -> ContractEvent {
        let adjusted_time = Some(convention.shift_bd(&schedule_time.unwrap()));
        ContractEvent {
            contractID: contract_id,
            currency: currency,
            eventTime: adjusted_time,
            eventType: event_type,
            payoff: pay_off,
            scheduleTime: schedule_time,
            state: state_trans
        }

    }

    /// Create a series of `ContractEvent`s from an unordered schedule of times
    pub fn create_events(
        event_schedule: &HashSet<IsoDatetime>,
        event_type: EventType,
        currency: Option<String>,
        pay_off: Rc<dyn TraitPayOffFunction>,
        state_trans: Rc<dyn TraitStateTransitionFunction>,
        contract_id: Option<String>,
    ) -> HashSet<ContractEvent> {
        event_schedule
            .iter()
            .map(|&time| {
                ContractEvent {
                    contractID: contract_id.clone(),
                    currency: currency.clone(),
                    eventTime: Some(time),
                    eventType: event_type,
                    payoff: pay_off.clone(),
                    scheduleTime: Some(time),
                    state: state_trans.clone(),
                }
            })
            .collect()
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
        pay_off: Rc<dyn TraitPayOffFunction>,
        state_trans: Rc<dyn TraitStateTransitionFunction>,
        convention: &BusinessDayConvention,
        contract_id: Option<String>,
    ) -> HashSet<ContractEvent> {
        event_schedule
            .iter()
            .map(|&time| {
                let adjusted_time = convention.shift_bd(&time);
                ContractEvent {
                    contractID: contract_id.clone(),
                    currency: currency.clone(),
                    eventTime: Some(adjusted_time),
                    eventType: event_type,
                    payoff: pay_off.clone(),
                    scheduleTime: Some(time),
                    state: state_trans.clone()
                }
            })
            .collect()
    }
}
