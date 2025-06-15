use std::fmt;
use std::rc::Rc;

pub type EventTime = IsoDatetime;
pub type ScheduleTime = IsoDatetime;

use std::hash::{Hash, Hasher};
use crate::events::EventType::EventType;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[derive(Clone)]
pub struct ContractEvent {
    pub contractID: Option<String>,
    pub currency: Option<String>,
    pub eventTime: Option<IsoDatetime>,
    pub eventType: EventType,
    pub payoff: Rc<dyn TraitPayOffFunction>,
    pub scheduleTime: Option<IsoDatetime>,
    pub state: Rc<dyn TraitStateTransitionFunction>,
}


// Implémentation manuelle de Debug pour ContractEvent
impl fmt::Debug for ContractEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractEvent")
            .field("contractID", &self.contractID)
            .field("currency", &self.currency)
            .field("eventTime", &self.eventTime)
            .field("eventType", &self.eventType)
            .field("payoff", &"<dyn PayOffFunctionTrait>")
            .field("scheduleTime", &self.scheduleTime)
            .field("state", &"<dyn StateTransitionFunctionTrait>")
            .finish()
    }
}

// Implémentation manuelle de PartialEq pour ContractEvent
impl PartialEq for ContractEvent {
    fn eq(&self, other: &Self) -> bool {
        self.contractID == other.contractID
            && self.currency == other.currency
            && self.eventTime == other.eventTime
            && self.eventType == other.eventType
            && self.scheduleTime == other.scheduleTime
            // Comparer les pointeurs des traits dynamiques (optionnel)
            && Rc::ptr_eq(&self.payoff, &other.payoff)
            && Rc::ptr_eq(&self.state, &other.state)
    }
}

impl Eq for ContractEvent {}

impl Hash for ContractEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contractID.hash(state);
        self.currency.hash(state);
        self.eventTime.hash(state);
        self.eventType.hash(state);
        self.scheduleTime.hash(state);

        // Hasher les pointeurs des traits dynamiques
        Rc::as_ptr(&self.payoff).hash(state);
        Rc::as_ptr(&self.state).hash(state);
    }
}