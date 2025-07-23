
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::marker::PhantomData;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
//use crate::events::AnyContractEvent::AnyContractEvent;

use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

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
        pay_off: Option<Rc<dyn TraitPayOffFunction + 'static>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction + 'static>>,
        convention: &Option<BusinessDayAdjuster>,
        contract_id: &Option<ContractID>
    ) -> ContractEvent<T1, T2>
    {
        let mut dd : Option<T2> = None;

        if convention.is_none() {
            let schedule_time_copy = schedule_time.clone().unwrap().value();
            dd = Some(T2::from(schedule_time_copy));
        }
        else {
            let time = schedule_time.as_ref().unwrap().value();
            let adjusted_time = convention.clone().unwrap().shift_bd(&time);
            let conventionx = Some(T2::from(adjusted_time));
            dd = conventionx.clone()
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
        event_schedule: &HashSet<T1>,
        event_type: &EventType,
        currency: &Option<Currency>,
        pay_off: Option<Rc<dyn TraitPayOffFunction>>,
        state_trans: Option<Rc<dyn TraitStateTransitionFunction>>,
        convention: &Option<BusinessDayAdjuster>,
        contract_id: &Option<ContractID>,
    ) -> HashSet<ContractEvent<T1, T2>> {
        
        event_schedule
            .iter()
            .map(|time| {
                let mut dd : Option<T2> = None;
                if convention.is_none() {
                    let schedule_time_copy = time.value();
                    dd = Some(T2::from(schedule_time_copy));
                    
                }
                else {
                    let timex = time.value();
                    let adjusted_time = convention.clone().unwrap().shift_bd(&timex);
                    let conventionx = Some(T2::from(adjusted_time));
                    dd = conventionx;
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

