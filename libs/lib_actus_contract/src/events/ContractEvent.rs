use std::fmt;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use lib_actus_terms::non_terms::EventTime::EventTime;
use lib_actus_terms::non_terms::PayOff::Payoff;
use lib_actus_terms::non_terms::ScheduleTime::ScheduleTime;
use lib_actus_terms::phantom_terms::PhantomF64::PhantomF64W;
use crate::events::EventSequence::EventSequence;
use crate::events::EventType::EventType;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

pub trait TraitContractEvent {}

#[derive(Clone)]
pub struct ContractEvent {
    
    pub epoch_offset: Option<PhantomF64W>,
    pub fstate: Option<StatesTransitionFunction>,
    pub fpayoff: Option<PayOffFunction>,
    pub event_time: Option<EventTime>,
    pub schedule_time: Option<ScheduleTime>,
    pub event_type: EventType,
    pub currency: Option<Currency>,
    pub payoff: Option<Payoff>,
    pub contract_id: Option<ContractID>,
}


impl TraitContractEvent for ContractEvent {}

impl ContractEvent {
    pub fn new (
        schedule_time: &Option<ScheduleTime>,
        event_time: &Option<EventTime>,
        event_type: &EventType,
        currency: &Option<Currency>,
        fpayoff: Option<PayOffFunction>,
        fstate: Option<StatesTransitionFunction>,
        contract_id: &Option<ContractID>,
    ) -> Self
    {

        let epoch_millis = event_time.clone().unwrap().value().and_utc().timestamp_millis(); //.and_utc().timestamp_millis();
        let epoch_offset = epoch_millis + EventSequence::time_offset(event_type);

        Self {
            epoch_offset: Some(PhantomF64W::new(epoch_offset as f64).unwrap() ),
            fstate: fstate,
            fpayoff: fpayoff,
            event_time: event_time.clone(),
            schedule_time: schedule_time.clone(),
            event_type: event_type.clone(),
            currency: currency.clone(),
            payoff: Some(Payoff::new(0.0).expect("ok")),
            contract_id: contract_id.clone(),
        }
    }

    pub fn get_contract_id(&self) -> ContractID {
        self.contract_id.clone().unwrap()
    }
    pub fn get_event_time(&self) -> EventTime {
        self.event_time.clone().unwrap()
    }
    pub fn get_schedule_time(&self) -> ScheduleTime {
        self.schedule_time.clone().unwrap()
    }
    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }
    pub fn chg_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
        // this.epoch_offset = event_time.toEpochSecond(ZoneOffset.UTC) + EventSequence.timeOffset(event_type);
        let a = self.get_event_time().timestamp_millis() as f64 + EventSequence::time_offset(&event_type) as f64;
        self.epoch_offset = Some(PhantomF64W::new(a).unwrap() );
    }
    pub fn currency(&self) -> Currency {
        self.currency.clone().unwrap()
    }
    pub fn payoff(&self) -> Payoff {
        self.payoff.clone().unwrap()
    }

    pub fn set_payoff(&mut self, payoff: f64) {
        self.payoff = Some( Payoff::new(payoff).expect("ok")  );
    }

    pub fn set_f_pay_off(&mut self, function: Option<PayOffFunction>) {
        self.fpayoff = function;
    }
    // Méthode pour changer fStateTrans
    pub fn set_f_state_trans(&mut self, function: Option<StatesTransitionFunction>) {
        self.fstate = function;
    }
    pub fn compare_to(&self, other: &ContractEvent) -> i64 {
        (self.epoch_offset.clone().unwrap() - other.epoch_offset.clone().unwrap()).signum() as i64
    }
  
    pub fn copy(&self) -> Self {
        ContractEvent {
            epoch_offset: self.epoch_offset.clone(),
            fstate: self.fstate.clone(),
            fpayoff: self.fpayoff.clone(),
            event_time: self.event_time.clone(),
            schedule_time: self.schedule_time.clone(),
            event_type: self.event_type.clone(),
            currency: self.currency.clone(),
            payoff: self.payoff,
            contract_id: self.contract_id.clone(),
        }
    }
    // Méthode pour obtenir une représentation sous forme de chaîne de caractères
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {:?} {} {}",
            self.epoch_offset.clone().unwrap(),
            self.get_event_time(),
            self.get_schedule_time(),
            self.event_type,
            self.currency.as_ref().unwrap().value().to_string(),
            self.payoff.unwrap(),
        )
    }


}

// Implémentation manuelle de Debug pour ContractEvent
impl Debug for ContractEvent
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractEvent")
            .field("contract_id", &self.contract_id)
            .field("currency", &self.currency)
            .field("event_time", &self.event_time)
            .field("event_type", &self.event_type)
            .field("payoff", &self.payoff)
            .field("schedule_time", &self.schedule_time)
            .finish()
    }
}


impl PartialOrd for ContractEvent
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ContractEvent
{
    fn cmp(&self, other: &Self) -> Ordering {
        // self.epoch_offset.unwrap().cmp(&other.epoch_offset.unwrap())
        self.epoch_offset.as_ref().unwrap().value().partial_cmp(&other.epoch_offset.as_ref().unwrap().value()).unwrap()
    }
}


impl PartialEq for ContractEvent
{
    fn eq(&self, other: &Self) -> bool {
        // Comparaison des champs standards
        let base_eq = self.contract_id == other.contract_id
            && self.currency == other.currency
            && self.event_time == other.event_time
            && self.event_type == other.event_type
            && self.schedule_time == other.schedule_time;

        // // Comparaison des fonctions avec gestion des None
        // let fpayoff_eq = match (&self.fpayoff, &other.fpayoff) {
        //     (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        //     (None, None) => true,
        //     _ => false,
        // };
        //
        // let fstate_eq = match (&self.fstate, &other.fstate) {
        //     (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        //     (None, None) => true,
        //     _ => false,
        // };


        // Comparaison des fonctions avec gestion des None
        let fpayoff_eq = match (&self.fpayoff, &other.fpayoff) {
            (Some(a), Some(b)) => std::ptr::eq(a, b), // ou utilisez une autre méthode de comparaison
            (None, None) => true,
            _ => false,
        };

        let fstate_eq = match (&self.fstate, &other.fstate) {
            (Some(a), Some(b)) => std::ptr::eq(a, b), // ou utilisez une autre méthode de comparaison
            (None, None) => true,
            _ => false,
        };


        base_eq && fpayoff_eq && fstate_eq
    }
}

impl Eq for ContractEvent
{}

impl Hash for ContractEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        //self.epoch_offset.hash(state);
        //self.fstate.hash(state);
        //self.fpayoff.hash(state);
        self.event_time.hash(state);
        self.schedule_time.hash(state);
        self.event_type.hash(state);
        self.currency.hash(state);
        //self.payoff.hash(state);
        self.contract_id.hash(state);
    }
}