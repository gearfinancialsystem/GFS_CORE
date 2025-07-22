use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

//pub type EventTime = IsoDatetime;
//pub type ScheduleTime = IsoDatetime;

use std::hash::{Hash, Hasher};
use crate::events::EventSequence::EventSequence;
use crate::events::EventType::EventType;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;


use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

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
        fpayoff: Option<Rc<dyn TraitPayOffFunction + 'static >>,
        fstate: Option<Rc<dyn TraitStateTransitionFunction + 'static >>,
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

    pub fn set_payoff(&mut self, payoff: f64) {
        self.payoff = Some(payoff);
    }

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
    // pub fn eval(
    //     &mut self,
    //     states: &mut StatesSpace,
    //     model: &ContractTerms,
    //     risk_factor_model: &RiskFactorModel,
    //     day_counter: &Option<DayCountConvention>,
    //     time_adjuster: &BusinessDayAdjuster) {
    //     if self.fpayoff.is_some() {
    //         self.payoff = Some(self.fpayoff.clone().unwrap().eval(
    //             &self.get_schedule_time(),
    //             states,
    //             model,
    //             risk_factor_model,
    //             day_counter,
    //             time_adjuster,
    //         ));
    //     }
    //     if self.fstate.is_some() {
    //         self.fstate.clone().unwrap().eval(
    //             &self.get_schedule_time(),
    //             states,
    //             model,
    //             risk_factor_model,
    //             day_counter,
    //             time_adjuster,
    //         );
    //     }
    // }

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
            contract_id: self.contract_id.clone(),
        }
    }
    // Méthode pour obtenir une représentation sous forme de chaîne de caractères
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {:?} {} {}",
            self.epoch_offset.unwrap(),
            self.get_event_time(),
            self.get_schedule_time(),
            self.event_type,
            self.currency.as_ref().unwrap().value().to_string(),
            self.payoff.unwrap(),
        )
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