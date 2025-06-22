use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type EventTime = IsoDatetime;
pub type ScheduleTime = IsoDatetime;

use std::hash::{Hash, Hasher};
use crate::attributes::ContractModel::ContractModel;
use crate::events::EventSequence::EventSequence;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;


//
// Lorsque vous appelez sorted() sur une liste de ContractEvent, la méthode utilise l'ordre naturel défini par l'interface Comparable<ContractEvent>, qui est implémentée par la classe ContractEvent. Plus précisément, le tri est effectué en utilisant la méthode compareTo de la classe ContractEvent, qui compare les événements basés sur leur epochOffset.
//
// Ainsi, les événements sont triés dans l'ordre croissant de leur epochOffset, ce qui correspond à un ordre chronologique basé sur le temps de l'événement et un décalage spécifique au type d'événement.
//
// Réponse finale :
//
// La méthode sorted() utilise l'ordre naturel défini par la méthode compareTo de la classe ContractEvent. Les événements sont triés selon leur epochOffset, qui est basé sur le temps de l'événement et un décalage spécifique au type d'événement.
//

#[derive(Clone)]
pub struct ContractEvent {
    pub epochOffset: Option<i64>,
    pub fstate: Option<Rc<dyn TraitStateTransitionFunction>>,
    pub fpayoff: Option<Rc<dyn TraitPayOffFunction>>,
    pub eventTime: Option<IsoDatetime>,
    pub scheduleTime: Option<IsoDatetime>,
    pub eventType: EventType,
    pub currency: Option<String>,
    pub payoff: Option<f64>,
    pub state: StateSpace,
    pub contractID: Option<String>,
}

impl ContractEvent {
    pub fn new(
        schedule_time: Option<IsoDatetime>,
        event_time: Option<IsoDatetime>,
        event_type: EventType,
        currency: Option<String>,
        fpayoff: Option<Rc<dyn TraitPayOffFunction>>,
        fstate: Option<Rc<dyn TraitStateTransitionFunction>>,
        contract_id: Option<String>,
    ) -> Self {
        let epoch_millis = event_time.unwrap().and_utc().timestamp_millis();
        let epoch_offset = epoch_millis + EventSequence::time_offset(event_type);

        Self {
            epochOffset: Some(epoch_offset),
            fstate: fstate,
            fpayoff: fpayoff,
            eventTime: event_time,
            scheduleTime: schedule_time,
            eventType: event_type,
            currency: currency,
            payoff: Some(0.0),
            state: StateSpace::default(),
            contractID: contract_id,
        }
    }

    pub fn get_contractID(&self) -> String {
        self.contractID.clone().unwrap()
    }
    pub fn get_event_time(&self) -> IsoDatetime {
        self.eventTime.clone().unwrap()
    }
    pub fn get_eventType(&self) -> EventType {
        self.eventType
    }
    pub fn chg_eventType(&mut self, eventType: EventType) {
        self.eventType = eventType;
        // this.epochOffset = eventTime.toEpochSecond(ZoneOffset.UTC) + EventSequence.timeOffset(eventType);
        self.epochOffset = Some(self.eventTime.unwrap().and_utc().timestamp_millis() + EventSequence::time_offset(eventType));
    }
    pub fn currency(&self) -> String {
        self.currency.clone().unwrap()
    }

    pub fn payoff(&self) -> f64 {
        self.payoff.clone().unwrap()
    }

    pub fn  states(&self) -> StateSpace {
        self.state.clone()
    }

    pub fn setStates(&mut self, state: StateSpace) {
        self.state = state;
    }

    // Méthode pour changer fPayOff
    pub fn set_f_pay_off(&mut self, function: Option<Rc<dyn TraitPayOffFunction>>) {
        self.fpayoff = function;
    }

    // Méthode pour changer fStateTrans
    pub fn set_f_state_trans(&mut self, function: Option<Rc<dyn TraitStateTransitionFunction>>) {
        self.fstate = function;
    }

    pub fn compare_to(&self, other: &Self) -> i64 {
        (self.epochOffset.unwrap() - other.epochOffset.unwrap()).signum()
    }

    pub fn eval(
        &mut self,
        states: &mut StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        if !self.fpayoff.is_none() {
            self.payoff = Some(self.fpayoff.clone().unwrap().eval(
                &self.scheduleTime.unwrap(),
                states,
                model,
                risk_factor_model,
                day_counter,
                time_adjuster,
            ));
        }
        if !self.fstate.is_none() {
            self.fstate.clone().unwrap().eval( // a verifier
                              &self.scheduleTime.unwrap(),
                              states,
                              model,
                              risk_factor_model,
                              day_counter,
                              time_adjuster,
            );
        }

    }

    pub fn copy(&self) -> Self {
        ContractEvent {
            epochOffset: self.epochOffset,
            fstate: self.fstate.clone(),
            fpayoff: self.fpayoff.clone(),
            eventTime: self.eventTime,
            scheduleTime: self.scheduleTime,
            eventType: self.eventType.clone(),
            currency: self.currency.clone(),
            payoff: self.payoff,
            state: self.state.clone(),
            contractID: self.contractID.clone(),
        }
    }
    // Méthode pour obtenir une représentation sous forme de chaîne de caractères
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {:?} {} {} {:?}",
            self.epochOffset.unwrap(),
            self.eventTime.unwrap(),
            self.scheduleTime.unwrap(),
            self.eventType,
            self.currency.clone().unwrap(),
            self.payoff.unwrap(),
            self.state
        )
    }
    // Méthode pour obtenir toutes les variables d'état sous forme de dictionnaire
    pub fn get_all_states(&self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        attributes.insert("payoff".to_string(), self.payoff.unwrap().to_string());
        attributes.insert("currency".to_string(), self.currency.clone().unwrap());
        attributes.insert("eventDate".to_string(), self.eventTime.unwrap().to_string());
        attributes.insert("eventType".to_string(), format!("{:?}", self.eventType));
        // Ajoutez d'autres attributs ici en fonction des champs de StateSpace
        if let Some(value) = self.state.accruedInterest {
            attributes.insert("accruedInterest".to_string(), value.to_string());
        }
        if let Some(value) = self.state.accruedInterest2 {
            attributes.insert("accruedInterest2".to_string(), value.to_string());
        }
        if let Some(value) = self.state.exerciseAmount {
            attributes.insert("exerciseAmount".to_string(), value.to_string());
        }
        if let Some(value) = self.state.exerciseDate {
            attributes.insert("exerciseDate".to_string(), value.to_string());
        }
        if let Some(value) = self.state.feeAccrued {
            attributes.insert("feeAccrued".to_string(), value.to_string());
        }
        if let Some(value) = self.state.interestCalculationBaseAmount {
            attributes.insert("interestCalculationBaseAmount".to_string(), value.to_string());
        }
        if let Some(value) = self.state.interestScalingMultiplier {
            attributes.insert("interestScalingMultiplier".to_string(), value.to_string());
        }
        if let Some(value) = self.state.nextPrincipalRedemptionPayment {
            attributes.insert("nextPrincipalRedemptionPayment".to_string(), value.to_string());
        }
        if let Some(value) = self.state.nominalInterestRate {
            attributes.insert("nominalInterestRate".to_string(), value.to_string());
        }
        if let Some(value) = self.state.nominalInterestRate2 {
            attributes.insert("nominalInterestRate2".to_string(), value.to_string());
        }
        if let Some(value) = self.state.nonPerformingDate {
            attributes.insert("nonPerformingDate".to_string(), value.to_string());
        }
        if let Some(value) = self.state.notionalPrincipal {
            attributes.insert("notionalPrincipal".to_string(), value.to_string());
        }
        if let Some(value) = self.state.notionalPrincipal2 {
            attributes.insert("notionalPrincipal2".to_string(), value.to_string());
        }
        if let Some(value) = self.state.notionalScalingMultiplier {
            attributes.insert("notionalScalingMultiplier".to_string(), value.to_string());
        }
        if let Some(value) = self.state.lastInterestPeriod {
            attributes.insert("lastInterestPeriod".to_string(), value.to_string());
        }
        attributes
    }

}

// Implémentation manuelle de Debug pour ContractEvent
impl fmt::Debug for ContractEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractEvent")
            .field("contractID", &self.contractID)
            .field("currency", &self.currency)
            .field("eventTime", &self.eventTime)
            .field("eventType", &self.eventType)
            .field("payoff", &self.payoff)
            .field("scheduleTime", &self.scheduleTime)
            .field("state", &self.state)
            .finish()
    }
}

// Implémentation des traits pour la comparaison
use std::cmp::Ordering;
impl PartialOrd for ContractEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ContractEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.epochOffset.cmp(&other.epochOffset)
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
            && Rc::ptr_eq(&self.fpayoff.clone().unwrap(), &other.fpayoff.clone().unwrap())
            && Rc::ptr_eq(&self.fstate.clone().unwrap(), &other.fstate.clone().unwrap())
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
        Rc::as_ptr(&self.fpayoff.clone().unwrap()).hash(state);
        Rc::as_ptr(&self.fstate.clone().unwrap()).hash(state);
    }
}