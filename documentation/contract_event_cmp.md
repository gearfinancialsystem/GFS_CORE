use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::collections::HashMap;

// Types et enums nécessaires
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
AD,
RRF,
// Ajoutez d'autres types d'événements ici
}

#[derive(Debug, Clone)]
pub struct StateSpace {
pub accrued_interest: f64,
pub accrued_interest2: f64,
// Ajoutez d'autres champs ici
}

impl Default for StateSpace {
fn default() -> Self {
StateSpace {
accrued_interest: 0.0,
accrued_interest2: 0.0,
}
}
}

// Traits pour les fonctions
pub trait StateTransitionFunction {
fn eval(
&self,
schedule_time: DateTime<Utc>,
states: &StateSpace,
model: &ContractModelProvider,
risk_factor_model: &RiskFactorModelProvider,
day_counter: &DayCountCalculator,
time_adjuster: &BusinessDayAdjuster,
) -> StateSpace;
}

pub trait PayOffFunction {
fn eval(
&self,
schedule_time: DateTime<Utc>,
states: &StateSpace,
model: &ContractModelProvider,
risk_factor_model: &RiskFactorModelProvider,
day_counter: &DayCountCalculator,
time_adjuster: &BusinessDayAdjuster,
) -> f64;
}

// Structures factices pour les autres types
pub struct ContractModelProvider;
pub struct RiskFactorModelProvider;
pub struct DayCountCalculator;
pub struct BusinessDayAdjuster;

// Implémentation d'exemple pour StateTransitionFunction et PayOffFunction
#[derive(Clone)]
struct STF_RRF_PAM;
impl StateTransitionFunction for STF_RRF_PAM {
fn eval(
&self,
_schedule_time: DateTime<Utc>,
_states: &StateSpace,
_model: &ContractModelProvider,
_risk_factor_model: &RiskFactorModelProvider,
_day_counter: &DayCountCalculator,
_time_adjuster: &BusinessDayAdjuster,
) -> StateSpace {
// Implémentez la logique réelle ici
StateSpace::default()
}
}

#[derive(Clone)]
struct ExamplePayOffFunction;
impl PayOffFunction for ExamplePayOffFunction {
fn eval(
&self,
_schedule_time: DateTime<Utc>,
_states: &StateSpace,
_model: &ContractModelProvider,
_risk_factor_model: &RiskFactorModelProvider,
_day_counter: &DayCountCalculator,
_time_adjuster: &BusinessDayAdjuster,
) -> f64 {
// Implémentez la logique réelle ici
0.0
}
}

// Définition de ContractEvent
#[derive(Debug, Clone)]
pub struct ContractEvent {
pub epoch_offset: i64,
pub event_time: DateTime<Utc>,
pub schedule_time: DateTime<Utc>,
pub event_type: EventType,
pub currency: String,
pub payoff: f64,
pub states: StateSpace,
pub contract_id: String,
pub f_state_trans: Box<dyn StateTransitionFunction>,
pub f_pay_off: Box<dyn PayOffFunction>,
}

impl ContractEvent {
pub fn new(
schedule_time: DateTime<Utc>,
event_time: DateTime<Utc>,
event_type: EventType,
currency: String,
f_pay_off: Box<dyn PayOffFunction>,
f_state_trans: Box<dyn StateTransitionFunction>,
contract_id: String,
) -> Self {
let epoch_offset = event_time.timestamp_millis() + Self::time_offset(&event_type);
ContractEvent {
epoch_offset,
event_time,
schedule_time,
event_type,
currency,
payoff: 0.0,
states: StateSpace::default(),
contract_id,
f_state_trans,
f_pay_off,
}
}

    pub fn time_offset(event_type: &EventType) -> i64 {
        match event_type {
            EventType::AD => 0,
            EventType::RRF => 1,
            // Ajoutez d'autres cas ici
        }
    }

    pub fn eval(
        &mut self,
        states: &StateSpace,
        model: &ContractModelProvider,
        risk_factor_model: &RiskFactorModelProvider,
        day_counter: &DayCountCalculator,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        self.payoff = self.f_pay_off.eval(
            self.schedule_time,
            states,
            model,
            risk_factor_model,
            day_counter,
            time_adjuster,
        );
        self.states = self.f_state_trans.eval(
            self.schedule_time,
            states,
            model,
            risk_factor_model,
            day_counter,
            time_adjuster,
        );
    }

    pub fn copy(&self) -> Self {
        ContractEvent {
            epoch_offset: self.epoch_offset,
            event_time: self.event_time,
            schedule_time: self.schedule_time,
            event_type: self.event_type.clone(),
            currency: self.currency.clone(),
            payoff: self.payoff,
            states: self.states.clone(),
            contract_id: self.contract_id.clone(),
            f_state_trans: self.f_state_trans.clone_box(),
            f_pay_off: self.f_pay_off.clone_box(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}, {}, {}, {:?}, {}, {}, {}",
            self.epoch_offset,
            self.event_time,
            self.schedule_time,
            self.event_type,
            self.currency,
            self.payoff,
            self.contract_id
        )
    }

    pub fn get_all_states(&self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        attributes.insert("payoff".to_string(), self.payoff.to_string());
        attributes.insert("currency".to_string(), self.currency.clone());
        attributes.insert("eventDate".to_string(), self.event_time.to_string());
        attributes.insert("eventType".to_string(), format!("{:?}", self.event_type));
        // Ajoutez d'autres attributs ici
        attributes
    }
}

// Implémentation des traits pour la comparaison
impl PartialEq for ContractEvent {
fn eq(&self, other: &Self) -> bool {
self.epoch_offset == other.epoch_offset
}
}

impl Eq for ContractEvent {}

impl PartialOrd for ContractEvent {
fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
Some(self.cmp(other))
}
}

impl Ord for ContractEvent {
fn cmp(&self, other: &Self) -> Ordering {
self.epoch_offset.cmp(&other.epoch_offset)
}
}

// Clone pour les Box<dyn Trait>
trait CloneBox {
fn clone_box(&self) -> Box<dyn StateTransitionFunction>;
}

impl<T: StateTransitionFunction + Clone + 'static> CloneBox for T {
fn clone_box(&self) -> Box<dyn StateTransitionFunction> {
Box::new(self.clone())
}
}

impl Clone for Box<dyn StateTransitionFunction> {
fn clone(&self) -> Self {
self.clone_box()
}
}

trait ClonePayOffBox {
fn clone_box(&self) -> Box<dyn PayOffFunction>;
}

impl<T: PayOffFunction + Clone + 'static> ClonePayOffBox for T {
fn clone_box(&self) -> Box<dyn PayOffFunction> {
Box::new(self.clone())
}
}

impl Clone for Box<dyn PayOffFunction> {
fn clone(&self) -> Self {
self.clone_box()
}
}

fn main() {
// Exemple d'utilisation
let schedule_time = Utc::now();
let event_time = Utc::now();
let contract_id = "contract1".to_string();
let currency = "USD".to_string();
let f_pay_off = Box::new(ExamplePayOffFunction);
let f_state_trans = Box::new(STF_RRF_PAM);

    let mut event = ContractEvent::new(
        schedule_time,
        event_time,
        EventType::AD,
        currency,
        f_pay_off,
        f_state_trans,
        contract_id,
    );

    // Exemple d'évaluation
    let states = StateSpace::default();
    let model = ContractModelProvider;
    let risk_factor_model = RiskFactorModelProvider;
    let day_counter = DayCountCalculator;
    let time_adjuster = BusinessDayAdjuster;

    event.eval(&states, &model, &risk_factor_model, &day_counter, &time_adjuster);

    println!("Event: {:?}", event);

    // Exemple de tri
    let mut events = vec![
        ContractEvent::new(
            Utc::now(),
            Utc::now(),
            EventType::AD,
            "USD".to_string(),
            Box::new(ExamplePayOffFunction),
            Box::new(STF_RRF_PAM),
            "contract1".to_string(),
        ),
        ContractEvent::new(
            Utc::now(),
            Utc::now(),
            EventType::RRF,
            "EUR".to_string(),
            Box::new(ExamplePayOffFunction),
            Box::new(STF_RRF_PAM),
            "contract2".to_string(),
        ),
    ];

    events.sort();

    for event in events {
        println!("Sorted Event: {}", event.to_string());
    }
}
