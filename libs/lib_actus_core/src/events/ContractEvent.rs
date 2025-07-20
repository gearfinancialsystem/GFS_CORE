use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type EventTime = IsoDatetime;
pub type ScheduleTime = IsoDatetime;

use std::hash::{Hash, Hasher};
use crate::events::EventSequence::EventSequence;
use crate::events::EventType::EventType;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::attributes::ContractTerms::ContractTerms;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

pub trait TraitContractEvent {}

#[derive(Clone)]
pub struct ContractEvent<T1, T2> {
    pub _marker_t1: PhantomData<T1>,
    pub _marker_t2: PhantomData<T2>,

    pub epoch_offset: Option<i64>,
    pub fstate: Option<Rc<dyn TraitStateTransitionFunction>>,
    pub fpayoff: Option<Rc<dyn TraitPayOffFunction>>,
    pub event_time: Option<T2>,
    pub schedule_time: Option<T1>,
    pub event_type: EventType,
    pub currency: Option<Currency>,
    pub payoff: Option<f64>,
    pub state: StateSpace,
    pub contract_id: Option<ContractID>,
}



impl<T1, T2> TraitContractEvent for ContractEvent<T1, T2> {}

impl<T1, T2> ContractEvent<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash + From<IsoDatetime>,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash + From<IsoDatetime>,
{
    pub fn new (
        schedule_time: &Option<T1>,
        event_time: &Option<T2>,
        event_type: &EventType,
        currency: &Option<Currency>,
        fpayoff: Option<Rc<dyn TraitPayOffFunction>>,
        fstate: Option<Rc<dyn TraitStateTransitionFunction>>,
        contract_id: &Option<ContractID>,
    ) -> Self
    {
        let epoch_millis = event_time.clone().unwrap().value().and_utc().timestamp_millis(); //.and_utc().timestamp_millis();
        let epoch_offset = epoch_millis + EventSequence::time_offset(event_type);

        Self {
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
            epoch_offset: Some(epoch_offset),
            fstate: fstate,
            fpayoff: fpayoff,
            event_time: event_time.clone(),
            schedule_time: schedule_time.clone(),
            event_type: event_type.clone(),
            currency: currency.clone(),
            payoff: Some(0.0),
            state: StateSpace::default(),
            contract_id: contract_id.clone(),
        }
    }
    pub fn to_iso_datetime_event(&self) -> ContractEvent<IsoDatetime, IsoDatetime> {
        ContractEvent {
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
            epoch_offset: self.epoch_offset,
            fstate: self.fstate.clone(),
            fpayoff: self.fpayoff.clone(),
            event_time: self.event_time.clone().map(|t| t.value()),
            schedule_time: self.schedule_time.clone().map(|t| t.value()),
            event_type: self.event_type.clone(),
            currency: self.currency.clone(),
            payoff: self.payoff,
            state: self.state.clone(),
            contract_id: self.contract_id.clone(),
        }
    }

    pub fn get_contract_id(&self) -> ContractID {
        self.contract_id.clone().unwrap()
    }
    pub fn get_event_time(&self) -> IsoDatetime {
        self.event_time.clone().unwrap().value()
    }
    pub fn get_schedule_time(&self) -> IsoDatetime {
        self.schedule_time.clone().unwrap().value()
    }
    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }
    pub fn chg_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
        // this.epoch_offset = event_time.toEpochSecond(ZoneOffset.UTC) + EventSequence.timeOffset(event_type);
        self.epoch_offset = Some(self.get_event_time().and_utc().timestamp_millis() + EventSequence::time_offset(&event_type));
    }
    pub fn currency(&self) -> Currency {
        self.currency.clone().unwrap()
    }
    pub fn payoff(&self) -> f64 {
        self.payoff.clone().unwrap()
    }
    pub fn states(&self) -> StateSpace {
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
    pub fn compare_to(&self, other: &ContractEvent<T1, T2>) -> i64 {
        (self.epoch_offset.unwrap() - other.epoch_offset.unwrap()).signum()
    }
    pub fn eval(
        &mut self,
        states: &mut StateSpace,
        model: &ContractTerms,
        risk_factor_model: &DataObserver,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster) {
        if self.fpayoff.is_some() {
            self.payoff = Some(self.fpayoff.clone().unwrap().eval(
                &self.get_schedule_time(),
                states,
                model,
                risk_factor_model,
                day_counter,
                time_adjuster,
            ));
        }
        if self.fstate.is_some() {
            self.fstate.clone().unwrap().eval(
                &self.get_schedule_time(),
                states,
                model,
                risk_factor_model,
                day_counter,
                time_adjuster,
            );
        }
    }

    // pub fn copy(&self) -> Self {
    //     ContractEvent {
    //         _marker_t1: PhantomData,
    //         _marker_t2: PhantomData,
    //         epoch_offset: self.epoch_offset,
    //         fstate: self.fstate.clone(),
    //         fpayoff: self.fpayoff.clone(),
    //         event_time: self.event_time.clone(),
    //         schedule_time: self.schedule_time.clone(),
    //         event_type: self.event_type.clone(),
    //         currency: self.currency.clone(),
    //         payoff: self.payoff,
    //         state: self.state.clone(),
    //         contract_id: self.contract_id.clone(),
    //     }
    // }

    pub fn copy(&self) -> Self {
        ContractEvent {
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
            epoch_offset: self.epoch_offset,
            fstate: self.fstate.clone(),
            fpayoff: self.fpayoff.clone(),
            event_time: self.event_time.clone(),
            schedule_time: self.schedule_time.clone(),
            event_type: self.event_type.clone(),
            currency: self.currency.clone(),
            payoff: self.payoff,
            state: self.state.clone(),
            contract_id: self.contract_id.clone(),
        }
    }
    // Méthode pour obtenir une représentation sous forme de chaîne de caractères
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {:?} {} {} {:?}",
            self.epoch_offset.unwrap(),
            self.get_event_time(),
            self.get_schedule_time(),
            self.event_type,
            self.currency.as_ref().unwrap().value().to_string(),
            self.payoff.unwrap(),
            self.state
        )
    }
    // Méthode pour obtenir toutes les variables d'état sous forme de dictionnaire
    pub fn get_all_states(&self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        attributes.insert("payoff".to_string(), self.payoff.as_ref().unwrap().to_string()   );
        attributes.insert("currency".to_string(), self.currency.as_ref().unwrap().value()   );
        attributes.insert("eventDate".to_string(), self.event_time.as_ref().unwrap().value().to_string() );
        attributes.insert("event_type".to_string(), format!("{:?}", self.event_type));
        // Ajoutez d'autres attributs ici en fonction des champs de StateSpace
        if let Some(value) = self.state.accrued_interest.clone(){
            attributes.insert("accrued_interest".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.accrued_interest2.clone()  {
            attributes.insert("accrued_interest2".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.exercise_amount.clone()  {
            attributes.insert("exercise_amount".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.exercise_date.clone()  {
            attributes.insert("exercise_date".to_string(), value.value().to_string());
        }
        if let Some(value) = self.state.fee_accrued.clone()  {
            attributes.insert("feeAccrued".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.interest_calculation_base_amount.clone()  {
            attributes.insert("interest_calculation_base_amount".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.interest_scaling_multiplier.clone() {
            attributes.insert("interest_scaling_multiplier".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.next_principal_redemption_payment.clone()  {
            attributes.insert("next_principal_redemption_payment".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.nominal_interest_rate.clone()  {
            attributes.insert("nominal_interest_rate".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.nominal_interest_rate2.clone()  {
            attributes.insert("nominal_interest_rate2".to_string(), value.to_string_rounded(2));
        }
        // ????
        if let Some(value) = self.state.non_performing_date.clone()  {
            attributes.insert("nonPerformingDate".to_string(), value.value().to_string());
        }

        if let Some(value) = self.state.notional_principal.clone()  {
            attributes.insert("notional_principal".to_string(), value.to_string_rounded(2));
        }
        if let Some(value) = self.state.notional_principal2.clone()  {
            attributes.insert("notional_principal2".to_string(), value.to_string_rounded(2));
        }


        if let Some(value) = self.state.notional_scaling_multiplier.clone()  {
            attributes.insert("notional_scaling_multiplier".to_string(), value.to_string_rounded(2));
        }

        // ????
        if let Some(value) = self.state.last_interest_period.clone()  {
            attributes.insert("lastInterestPeriod".to_string(), value.to_string()); // a refaire ici
        }
        attributes
    }
    pub fn get_computed_result(&self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();

        attributes.insert("event_date".to_string(), self.event_time.as_ref().unwrap().value().to_string() );
        attributes.insert("event_type".to_string(), format!("{:?}", self.event_type));
        attributes.insert("currency".to_string(), self.currency.as_ref().unwrap().value()   );
        attributes.insert("payoff".to_string(), self.payoff.as_ref().unwrap().to_string()   );
        
        // Ajoutez d'autres attributs ici en fonction des champs de StateSpace
        if let Some(value) = self.state.accrued_interest.clone(){
            attributes.insert("accrued_interest".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.accrued_interest2.clone()  {
            attributes.insert("accrued_interest2".to_string(), value.to_string_rounded(10));
        }
        
        if let Some(value) = self.state.exercise_amount.clone()  {
            attributes.insert("exercise_amount".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.exercise_date.clone()  {
            attributes.insert("exercise_date".to_string(), value.value().to_string());
        }
        
        if let Some(value) = self.state.fee_accrued.clone()  {
            attributes.insert("fee_accrued".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.interest_calculation_base_amount.clone()  {
            attributes.insert("interest_calculation_base_amount".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.interest_scaling_multiplier.clone() {
            attributes.insert("interest_scaling_multiplier".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.next_principal_redemption_payment.clone()  {
            attributes.insert("next_principal_redemption_payment".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.nominal_interest_rate.clone()  {
            attributes.insert("nominal_interest_rate".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.nominal_interest_rate2.clone()  {
            attributes.insert("nominal_interest_rate2".to_string(), value.to_string_rounded(10));
        }

        if let Some(value) = self.state.notional_principal.clone()  {
            attributes.insert("notional_principal".to_string(), value.to_string_rounded(10));
        }
        if let Some(value) = self.state.notional_principal2.clone()  {
            attributes.insert("notional_principal2".to_string(), value.to_string_rounded(10));
        }


        if let Some(value) = self.state.notional_scaling_multiplier.clone()  {
            attributes.insert("notional_scaling_multiplier".to_string(), value.to_string_rounded(10));
        }
        
        attributes
    }
}

// Implémentation manuelle de Debug pour ContractEvent
impl<T1, T2> Debug for ContractEvent<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractEvent")
            .field("contract_id", &self.contract_id)
            .field("currency", &self.currency)
            .field("event_time", &self.event_time)
            .field("event_type", &self.event_type)
            .field("payoff", &self.payoff)
            .field("schedule_time", &self.schedule_time)
            .field("state", &self.state)
            .finish()
    }
}

// Implémentation des traits pour la comparaison


impl<T1, T2> PartialOrd for ContractEvent<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T1, T2> Ord for ContractEvent<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.epoch_offset.cmp(&other.epoch_offset)
    }
}


impl<T1, T2> PartialEq for ContractEvent<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
{
    fn eq(&self, other: &Self) -> bool {
        // Comparaison des champs standards
        let base_eq = self.contract_id == other.contract_id
            && self.currency == other.currency
            && self.event_time == other.event_time
            && self.event_type == other.event_type
            && self.schedule_time == other.schedule_time;

        // Comparaison des fonctions avec gestion des None
        let fpayoff_eq = match (&self.fpayoff, &other.fpayoff) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };

        let fstate_eq = match (&self.fstate, &other.fstate) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };

        base_eq && fpayoff_eq && fstate_eq
    }
}
//
// // Implémentation manuelle de PartialEq pour ContractEvent
// impl<T1, T2> PartialEq for ContractEvent<T1, T2>
// where
//     T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
//     T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.contract_id == other.contract_id
//             && self.currency == other.currency
//             && self.event_time == other.event_time
//             && self.event_type == other.event_type
//             && self.schedule_time == other.schedule_time
//             // Comparer les pointeurs des traits dynamiques (optionnel)
//             && Rc::ptr_eq(&self.fpayoff.clone().unwrap(), &other.fpayoff.clone().unwrap())
//             && Rc::ptr_eq(&self.fstate.clone().unwrap(), &other.fstate.clone().unwrap())
//     }
// }
impl<T1, T2> Eq for ContractEvent<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
{}

// impl<T1, T2> Hash for ContractEvent<T1, T2>
// where
//     T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
//     T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
// {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.contract_id.hash(state);
//         self.currency.hash(state);
//         self.event_time.clone().hash(state);
//         self.event_type.hash(state);
//         self.schedule_time.hash(state);
//
//         // Hasher les pointeurs des traits dynamiques
//         Rc::as_ptr(&self.fpayoff.clone().unwrap()).hash(state);
//         Rc::as_ptr(&self.fstate.clone().unwrap()).hash(state);
//     }
// }

impl<T1, T2> Hash for ContractEvent<T1, T2>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hachage des champs standards
        self.contract_id.hash(state);
        self.currency.hash(state);
        self.event_time.hash(state);
        self.event_type.hash(state);
        self.schedule_time.hash(state);

        // Hachage des fonctions avec gestion des None
        if let Some(f) = &self.fpayoff {
            Rc::as_ptr(f).hash(state);
        } else {
            // Valeur sentinelle pour None
            0usize.hash(state);
        }

        if let Some(f) = &self.fstate {
            Rc::as_ptr(f).hash(state);
        } else {
            // Valeur sentinelle pour None
            0usize.hash(state);
        }
    }
}