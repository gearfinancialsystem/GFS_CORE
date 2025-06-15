use std::fmt;
use std::rc::Rc;

use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::event::EventType::EventType;
use crate::traits::PayOffFunctionTrait::PayOffFunctionTrait;
use crate::traits::StateTransitionFunctionTrait::StateTransitionFunctionTrait;

pub type EventTime = IsoDatetime;
pub type ScheduleTime = IsoDatetime;

use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct ContractEvent {
    pub contractID: ContractID,
    pub currency: Currency,
    pub eventTime: IsoDatetime,
    pub eventType: EventType,
    pub payoff: Rc<dyn PayOffFunctionTrait>,
    pub scheduleTime: IsoDatetime,
    pub state: Rc<dyn StateTransitionFunctionTrait>,
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