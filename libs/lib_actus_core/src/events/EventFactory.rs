
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::marker::PhantomData;
use crate::events::AnyContractEvent::AnyContractEvent;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

pub struct EventFactory<T1, T2> {
    _marker_t1: PhantomData<T1>,
    _marker_t2: PhantomData<T2>,
}

impl<T1, T2> EventFactory<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash + From<IsoDatetime>,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash + From<IsoDatetime>,
{
    // Créez une nouvelle instance de EventFactory
    pub fn new() -> Self {
        EventFactory {
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
    pub fn create_event(
        schedule_time: &Option<T1>,
        event_type: &EventType,
        currency: &Option<Currency>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        contract_id: &Option<ContractID>
    ) -> ContractEvent<T1, T2>
    {

        let schedule_time_copy = schedule_time.clone().unwrap().value();
        let dd = Some(T2::from(schedule_time_copy));

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

    /// Create a single `ContractEvent` with adjusted event time based on a business day convention
    pub fn create_event_with_convention(
        schedule_time: &Option<T1>,
        event_type: &EventType,
        currency: &Option<Currency>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        convention: &BusinessDayAdjuster,
        contract_id: &Option<ContractID>,

    ) -> ContractEvent<T1, T2> {
 
        let time = schedule_time.as_ref().unwrap().value();
        let adjusted_time = convention.shift_bd(&time);
        let conventionx = Some(T2::from(adjusted_time));
        
        ContractEvent::new(
            schedule_time,
            &conventionx,
            event_type,
            currency,
            pay_off,
            state_trans,
            contract_id,
        )

    }

    /// Create a series of `ContractEvent`s from an unordered schedule of times
    pub fn create_events(
        event_schedule: &HashSet<T1>,
        event_type: &EventType,
        currency: &Option<Currency>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        contract_id: &Option<ContractID>,
    ) -> HashSet<ContractEvent<T1, T2>> {
        
        event_schedule
            .iter()
            .map(|time| {
                let schedule_time_copy = time.value();
                let dd = Some(T2::from(schedule_time_copy));
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
        event_schedule: &HashSet<T1>,
        event_type: &EventType,
        currency: &Option<Currency>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        convention: &BusinessDayAdjuster,
        contract_id: &Option<ContractID>,
    ) -> HashSet<ContractEvent<T1, T2>> {
 
        event_schedule
            .iter()
            .map(|time| {
                //let adjusted_time = convention.shift_bd(&time);
                let timex = time.value();
                let adjusted_time = convention.shift_bd(&timex);
                let conventionx = Some(T2::from(adjusted_time));
                
                ContractEvent::new(
                    &Some(time.clone()),
                    &conventionx,
                    event_type,
                    currency,
                    pay_off.clone(),
                    state_trans.clone(),
                    contract_id,
                )
            })
            .collect()
    }
}
