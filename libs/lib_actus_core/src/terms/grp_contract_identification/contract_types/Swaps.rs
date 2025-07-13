
use std::fmt;
use std::rc::Rc;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::functions::swaps::pof::POF_NET_SWAPS::POF_NET_SWAPS;
use crate::functions::swaps::pof::POF_PRD_SWAPS::POF_PRD_SWAPS;
use crate::functions::swaps::pof::POF_TD_SWAPS::POF_TD_SWAPS;
use crate::functions::swaps::stf::STF_NET_SWAPS::STF_NET_SWAPS;
use crate::functions::swaps::stf::STF_PRD_SWAPS::STF_PRD_SWAPS;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::S::S;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;


pub struct SWAPS;

impl TraitContractModel for SWAPS {
    fn schedule(
        to: Option<IsoDatetime>,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {

        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();

        let (first_leg_model, second_leg_model) = SWAPS::get_legs_contract_models(model);

        let mat1 = first_leg_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();
        let mat2 = second_leg_model.maturity_date.clone().map(|rc| (*rc).clone()).unwrap().value();

        let first_leg_schedule: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = ContractType::schedule(Some(mat1), &first_leg_model).unwrap();
        let second_leg_schedule: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = ContractType::schedule(Some(mat2), &second_leg_model).unwrap();


        events.extend(first_leg_schedule);
        events.extend(second_leg_schedule);

        if model.purchase_date.is_some() {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(&model.purchase_date, // voir si le typage fort est correct
                                               &EventType::PRD,
                                               &model.currency,
                                               Some(Rc::new(POF_PRD_SWAPS)),
                                               Some(Rc::new(STF_PRD_SWAPS)),
                                               &None,
                                               &model.contract_id);
            events.push(e.to_iso_datetime_event());
        }

        if model.termination_date.is_some() {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &model.termination_date,
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_SWAPS)),
                Some(Rc::new(STF_TD_STK)),
                &None,
                &model.contract_id
            );
            events.retain(|e| e.compare_to(&termination.to_iso_datetime_event()) == 1);
            events.push(termination.to_iso_datetime_event());
        }
        //let a = &model.status_date;
        let ee: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::<StatusDate, StatusDate>::create_event(
            &model.status_date,
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id).to_iso_datetime_event();
        events.retain(|e| e.compare_to(&ee) != -1);

        events.retain(|e| e.compare_to(
            &EventFactory::create_event(
                &Some(to.clone().unwrap()),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id) ) != 1);
        Ok(events)
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    fn apply(
        events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {

        let mut events = events;
        let cs = model.clone().contract_structure.unwrap();
        let first_leg_model = cs.0.iter().filter(|cr| cr.reference_role == ReferenceRole::FIL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        let second_leg_model = cs.0.iter().filter(|cr| cr.reference_role == ReferenceRole::SEL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();

        let first_leg_schedule: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = events
            .iter()
            .filter(|event| {
                first_leg_model.contract_id.clone().unwrap() == event.get_contract_id()
            })
            .cloned()
            .collect();

        let second_leg_schedule: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = events.clone()
            .iter()
            .filter(|event| {
                second_leg_model.clone().contract_id.unwrap() == event.get_contract_id()
            })
            .cloned()
            .collect();


        // Remove the filtered events from the main events list
        events.retain(|e| {
            !first_leg_schedule.iter().any(|first_leg_event| first_leg_event.contract_id == e.contract_id) &&
                !second_leg_schedule.iter().any(|second_leg_event| second_leg_event.contract_id == e.contract_id)
        });

        // PROBLEM A REGLER ?? first_leg_schedule modifiee passer events en reference ?
        let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model.clone(), observer);
        let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model.clone(), observer);

        if model.delivery_settlement.clone().unwrap() == DeliverySettlement::S(S) {
            let a = SWAPS::filter_and_nett_congruent_events(
                first_leg_events.clone().unwrap(),
                second_leg_events.clone().unwrap(),
                model.contract_id.clone()
            );
            events.extend(a);
        } else {
            events.extend(first_leg_events.clone().unwrap());
            events.extend(second_leg_events.clone().unwrap());
        }

        events.iter().for_each(|event| {
            if event.get_contract_id() == model.contract_id.clone().unwrap() {
                if event.event_type == EventType::PRD || event.event_type == EventType::TD {
                    let mut parent_state = StateSpace::default();
                    let f_l_events_at_timepoint = first_leg_events.clone().unwrap().iter().filter(|e| {
                        e.event_time == event.event_time
                    }).map(|e| e.clone()).collect::<Vec<_>>();
                    let s_l_events_at_timepoint = second_leg_events.clone().unwrap().iter().filter(|e| {
                        e.event_time == event.event_time
                    }).map(|e| e.clone()).collect::<Vec<_>>();

                    let fl_ipac: f64;
                    let sl_ipac: f64;
                    if f_l_events_at_timepoint.is_empty() {
                        fl_ipac = 0.0;
                    }
                    else {
                        fl_ipac = if f_l_events_at_timepoint.iter().any(|e| e.event_type == EventType::IP) {
                            0.0
                        } else {
                            f_l_events_at_timepoint.iter()
                                .find(|e| e.event_type == EventType::PR)
                                .map(|e| e.states().accrued_interest.clone().unwrap().value())
                                .unwrap_or(0.0)
                        };
                    }
                    sl_ipac = if s_l_events_at_timepoint.is_empty() {
                        0.0
                    } else {
                        if s_l_events_at_timepoint.iter().any(|e| e.event_type == EventType::IP) {
                            0.0
                        } else {
                            s_l_events_at_timepoint.iter()
                                .find(|e| e.event_type == EventType::PR)
                                .map(|e| e.states().accrued_interest.clone().unwrap().value())
                                .unwrap_or(0.0)
                        }
                    };
                    parent_state.accrued_interest = AccruedInterest::new(fl_ipac + sl_ipac).ok();

                }
                else {
                    //event.clone().eval(None, None, None, None, None);
                    //A REFLECHIR
                }
            }
        });
        if model.purchase_date.is_some(){
            let purchase: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_SWAPS)),
                Some(Rc::new(STF_PRD_STK)), // WHY ?
                &None,
                &model.contract_id
            );
            // Remove the filtered events from the main events list
            events.retain(|e| {
                e.compare_to(&purchase.to_iso_datetime_event()) == -1
            });
        }
        events.sort();
        Ok(events)
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_state_space(
        model: &ContractModel, _observer: &RiskFactorModel, _maturity: &MaturityDate
    ) -> Result<StateSpace, String> { // event_at_t0: ContractEvent<IsoDatetime, IsoDatetime>,
        let cs = model.clone().contract_structure.unwrap();
        let first_leg_model = cs.0.iter().filter(|cr| cr.reference_role == ReferenceRole::FIL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        let second_leg_model = cs.0.iter().filter(|cr| cr.reference_role == ReferenceRole::SEL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();

        //let event_t0_status_date = event_at_t0.states().status_date;
        //let mut states = if event_t0_status_date.is_some() {
        //    StateSpace::default()
        //} else { event_at_t0.states() };
        let mut states = StateSpace::default();

        states.status_date = model.status_date.clone();
        states.contract_performance = if model.contract_performance.is_some() {
            model.contract_performance
        } else { None };
        let mat1 = first_leg_model.maturity_date.clone().map(|rc| (*rc).clone());
        let mat2 = second_leg_model.maturity_date.clone().map(|rc| (*rc).clone());
        states.maturity_date = if mat1.clone().unwrap().value() > mat2.clone().unwrap().value() { mat1 } else { mat2 };
        //states.accrued_interest = event_at_t0.states().accrued_interest;
        states.accrued_interest = AccruedInterest::new(0.0).ok();
        Ok(states)
    }


}

impl SWAPS {
    pub fn get_legs_contract_models(model: &ContractModel) -> (ContractModel, ContractModel) {

        let cs = model.clone().contract_structure.unwrap();

        let first_leg_model = cs.0.iter().filter(|cr| cr.reference_role == ReferenceRole::FIL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        let second_leg_model = cs.0.iter().filter(|cr| cr.reference_role == ReferenceRole::SEL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        (first_leg_model, second_leg_model)
    }
    /// Compute next events within the period up to `to` date based on the contract model

    pub fn filter_and_nett_congruent_events(
        first_leg_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        second_leg_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        parent_contract_ID: Option<ContractID>) -> Vec<ContractEvent<IsoDatetime, IsoDatetime>> {

        let mut first_leg_events = first_leg_events;
        let mut second_leg_events = second_leg_events;
        first_leg_events.sort_by(|a, b| a.event_time.cmp(&b.event_time));
        second_leg_events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        let mut events = Vec::new();

        // Helper function to filter events by type
        let filter_events = |events: &[ContractEvent<IsoDatetime, IsoDatetime>], event_type: EventType| -> Vec<ContractEvent<IsoDatetime, IsoDatetime>> {
            events.iter()
                .filter(|event| event.event_type == event_type)
                .cloned()
                .collect()
        };

        // Define a macro to reduce repetition for each event type
        macro_rules! process_event_type {
            ($event_type:expr) => {
                let mut first_leg_x = filter_events(&first_leg_events, $event_type);
                let mut second_leg_x = filter_events(&second_leg_events, $event_type);
                SWAPS::net_singular_event(
                    parent_contract_ID.clone(),
                    &mut events,
                    &mut first_leg_x,
                    &mut second_leg_x,
                );
            };
        }

        // Process IED and MD events (which use netSingularEvent)
        process_event_type!(EventType::IED);
        process_event_type!(EventType::MD);

        // Process PR events
        let first_leg_pr = filter_events(&first_leg_events, EventType::PR);
        let second_leg_pr = filter_events(&second_leg_events, EventType::PR);
        SWAPS::net_congruent_events(
            first_leg_pr,
            second_leg_pr,
            &mut events,
            parent_contract_ID.clone(),
        );

        // Process IP events
        let first_leg_ip = filter_events(&first_leg_events, EventType::IP);
        let second_leg_ip = filter_events(&second_leg_events, EventType::IP);
        SWAPS::net_congruent_events(
            first_leg_ip,
            second_leg_ip,
            &mut events,
            parent_contract_ID.clone(),
        );

        events
    }

    pub fn net_singular_event(parent_contract_id: Option<ContractID>,
                              events: &mut Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
                              list_first_leg: &mut Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
                              list_second_leg: &mut Vec<ContractEvent<IsoDatetime, IsoDatetime>>,){

        if !list_first_leg.is_empty() && !list_second_leg.is_empty() {
            let first_leg_event = &list_first_leg.clone()[0];
            let second_leg_event = &list_second_leg.clone()[0];

            if first_leg_event.event_time == second_leg_event.event_time {
                // Remove from events list - but we can't do exact object matching
                // So we'll use a more functional approach: filter events not matching our two
                let mut new_events = Vec::new();
                let mut first_found = false;
                let mut second_found = false;

                for event in events.drain(..) {
                    if !first_found && event.event_time == first_leg_event.event_time {
                        first_found = true;
                        continue; // Skip this event (don't add to new_events)
                    }
                    if !second_found && event.event_time == second_leg_event.event_time {
                        second_found = true;
                        continue; // Skip this event
                    }
                    new_events.push(event);
                }
                *events = new_events;

                // Remove the first element from both leg lists
                list_first_leg.remove(0);
                list_second_leg.remove(0);

                // Create and add the netting event
                let netting_event = SWAPS::netting_event(
                    Some(first_leg_event.clone()),
                    Some(second_leg_event.clone()),
                    parent_contract_id,
                );
                events.push(netting_event);
            }
        }
    }

    pub fn net_congruent_events(
        first_leg_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        second_leg_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        events: &mut Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
        parent_contract_ID: Option<ContractID>) {

        let mut first_leg = first_leg_events;
        let mut second_leg = second_leg_events;

        // Sort both lists by event time
        first_leg.sort_by(|a, b| a.event_time.cmp(&b.event_time));
        second_leg.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        let mut i = 0;
        let mut j = 0;

        // These will hold indices of events to remove
        let mut first_indices_to_remove = Vec::new();
        let mut second_indices_to_remove = Vec::new();

        // We'll iterate through both lists
        while i < first_leg.len() {
            // Check if there are more elements in second_leg
            while j < second_leg.len() {
                let first_event = &first_leg[i];
                let second_event = &second_leg[j];

                if first_event.event_time == second_event.event_time {
                    // Found matching events, create a netting event
                    let netting_event = SWAPS::netting_event(
                        Some(first_event.clone()),
                        Some(second_event.clone()),
                        parent_contract_ID.clone(),
                    );
                    events.push(netting_event);

                    // Mark these indices for removal
                    first_indices_to_remove.push(i);
                    second_indices_to_remove.push(j);

                    // Move both indices forward
                    i += 1;
                    j += 1;

                    // If we found a match, break out of inner loop
                    // equivalent to the 'break' in the Java code
                    break;
                } else if second_event.event_time < first_event.event_time {
                    // Second event is earlier, move to next in second leg
                    j += 1;
                } else {
                    // No match found for this first event, move to next
                    break;
                }
            }

            // If we didn't find a match after checking all second leg events
            // or if we're at the end of the second leg list
            if j >= second_leg.len() {
                i += 1;
                // Reset j to 0 for the next first leg event (though this might not match Java behavior)
                j = 0;
            }
        }

        // Remove marked events from first leg
        // Sort indices in descending order to avoid shifting issues when removing
        first_indices_to_remove.sort_unstable();
        first_indices_to_remove.dedup(); // in case of duplicates (shouldn't happen)
        for index in first_indices_to_remove.iter().rev() {
            if *index < first_leg.len() {
                first_leg.remove(*index);
            }
        }

        // Remove marked events from second leg
        second_indices_to_remove.sort_unstable();
        for index in second_indices_to_remove.iter().rev() {
            if *index < second_leg.len() {
                second_leg.remove(*index);
            }
        }

        // Add remaining events from both legs
        events.extend(first_leg);
        events.extend(second_leg);

    }

    pub fn netting_event(
        e1: Option<ContractEvent<IsoDatetime, IsoDatetime>>,
        e2: Option<ContractEvent<IsoDatetime, IsoDatetime>>,
        parent_contract_id: Option<ContractID>,
    ) -> ContractEvent<IsoDatetime, IsoDatetime> {
        let netting = EventFactory::create_event(
            &e1.clone().unwrap().event_time,
            &e1.clone().unwrap().event_type,
            &e1.clone().unwrap().currency,
            Some(Rc::new(POF_NET_SWAPS::new(e1.clone().unwrap(), e2.clone().unwrap()))),
            Some(Rc::new(STF_NET_SWAPS::new(e1.clone().unwrap(), e2.clone().unwrap()))),
            &None,
            &parent_contract_id.clone(),
        );
        netting
    }
}


impl fmt::Display for SWAPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SWAPS")
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::util_tests::TestsUtils::{convert_value_map_to_string_map_ref, test_read_and_parse_json};
//     use crate::util_tests::TestsUtils::json_to_dico;
//     use crate::util::Value::Value;
//     use std::error::Error;
//     use std::collections::{HashMap, HashSet};
//     use std::hash::Hash;
//     use chrono::ParseError;
//     use log::debug;
//     use crate::attributes::ContractReference::ContractReference;
//     use crate::exceptions::ContractTypeUnknownException::ContractError;
//
//
//     fn load_dico_tests() -> Vec<Value> {
//         let pathx = "/home/cet/Projects/ACTUS-CORE/actus-core-master-rust-project-v2/libs/lib_actus_core/tests_sets/actus-tests-swaps.json";
//         let json_value = test_read_and_parse_json(pathx).unwrap();
//         let dico_from_json = json_to_dico(json_value);
//         dico_from_json
//     }
//     fn are_contracts_equal(
//         mut contracts1: Vec<HashMap<String, Value>>,
//         mut contracts2: Vec<HashMap<String, Value>>
//     ) -> bool {
//         // 1. Vérifier que les vecteurs ont la même longueur
//         let a = contracts1.len();
//         let b = contracts2.len();
//
//         if a != b {
//             return false;
//         } else {
//             contracts1.sort_by(|a, b| {
//                 // Comparaison par eventTime (String) en premier
//                 let time_order = {
//                     let a_time = a.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     let b_time = b.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     match (a_time, b_time) {
//                         (Some(a), Some(b)) => a.cmp(b),
//                         (Some(_), None) => std::cmp::Ordering::Less,
//                         (None, Some(_)) => std::cmp::Ordering::Greater,
//                         (None, None) => std::cmp::Ordering::Equal,
//                     }
//                 };
//
//                 // Si les eventTime sont égaux, on compare par la valeur f64
//                 if time_order == std::cmp::Ordering::Equal {
//                     // Récupération des valeurs f64 avec gestion des erreurs
//                     let a_value: f64 = match a.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//                     let b_value: f64 = match b.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//
//                     // Comparaison des f64
//                     a_value.partial_cmp(&b_value).unwrap_or(std::cmp::Ordering::Equal)
//                 } else {
//                     time_order
//                 }
//             });
//             // contracts1.sort_by(|a, b| {
//             //     let a_time = a.get("eventTime").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     let b_time = b.get("eventTime").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     a_time.cmp(&b_time)
//             // });
//
//             contracts2.sort_by(|a, b| {
//                 // Comparaison par eventTime (String) en premier
//                 let time_order = {
//                     let a_time = a.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     let b_time = b.get("eventTime").and_then(|v| match v {
//                         Value::String(s) => Some(s.as_str()),
//                         _ => None
//                     });
//                     match (a_time, b_time) {
//                         (Some(a), Some(b)) => a.cmp(b),
//                         (Some(_), None) => std::cmp::Ordering::Less,
//                         (None, Some(_)) => std::cmp::Ordering::Greater,
//                         (None, None) => std::cmp::Ordering::Equal,
//                     }
//                 };
//
//                 // Si les eventTime sont égaux, on compare par la valeur f64
//                 if time_order == std::cmp::Ordering::Equal {
//                     // Récupération des valeurs f64 avec gestion des erreurs
//                     let a_value: f64 = match a.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//                     let b_value: f64 = match b.get("notionalPrincipal") { // Remplacez "amount" par votre clé
//                         Some(Value::F64(n)) => *n,
//                         _ => 0.0 // Valeur par défaut si la clé est manquante ou du mauvais type
//                     };
//
//                     // Comparaison des f64
//                     a_value.partial_cmp(&b_value).unwrap_or(std::cmp::Ordering::Equal)
//                 } else {
//                     time_order
//                 }
//             });
//             // contracts2.sort_by(|a, b| {
//             //     let a_time = a.get("eventDate").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     let b_time = b.get("eventDate").and_then(|v| {
//             //         if let Value::String(s) = v { Some(s) } else { None }
//             //     });
//             //     a_time.cmp(&b_time)
//             // });
//
//             let mut vec_bool: Vec<bool> = vec![];
//             let mut i = 0;
//             for hm in contracts1.into_iter() {
//                 for (k, v)  in hm.iter() {
//
//                     match k.as_str() {
//                         "eventDate" => {
//                             let w1 = hm.get(k.as_str()).unwrap().as_string().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().as_string().unwrap();
//                             if w1 == w2 {
//                                 vec_bool.push(true);
//                             }
//                             else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "eventType" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().as_string().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().as_string().unwrap();
//                             if w1 == w2 {
//                                 vec_bool.push(true);
//                             }
//                             else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "payoff" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "currency" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().as_string().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().as_string().unwrap();
//                             if w1 == w2 {
//                                 vec_bool.push(true);
//                             } else
//                             {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "notionalPrincipal" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 //let invest = hm.get("state");
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "nominalInterestRate" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         "accruedInterest" =>  {
//                             let w1 = hm.get(k.as_str()).unwrap().extract_f64().unwrap();
//                             let w2 = contracts2.get(i).unwrap().get(k.as_str()).unwrap().extract_f64().unwrap();
//                             if (w1 * 100.0).round() / 100.0 == (w2 * 100.0).round() / 100.0 {
//                                 vec_bool.push(true);
//                             } else {
//                                 vec_bool.push(false);
//                             }
//                         },
//                         _ => {}
//                     }
//                 }
//
//                 i = i + 1;
//             }
//
//             println!("{:?}", vec_bool);
//         }
//         true
//
//     }
//
//     #[test]
//     fn test_swap_contracts()  {
//
//         let dico_tests = load_dico_tests();
//
//         //let dico_tests: Vec<HashMap<String, Value>> = vec![load_dico_tests()];
//         for el in dico_tests.iter() {
//
//             let curr_test = el.extract_hmap().unwrap();
//
//             let curr_identifier = curr_test.get("identifier").unwrap().as_string();
//             let curr_terms = curr_test.get("terms").unwrap().extract_hmap();
//             let curr_to = curr_test.get("to").unwrap().as_string();
//             let curr_data_observed = curr_test.get("dataObserved").unwrap().extract_hmap(); // verifier si cest None
//             let curr_events_observed = curr_test.get("eventsObserved").unwrap().extract_vec();
//             let curr_results = curr_test.get("results").unwrap().extract_vec().unwrap();
//             //let a = curr_results.get(0).unwrap().get("notionalPrincipal").unwrap().as_string().unwrap();
//             let to_date = if let Some(curr_to) = curr_to {
//                 IsoDatetime::parse_from_str(&curr_to, "%Y-%m-%dT%H:%M:%S").ok()
//             } else {
//                 None
//             };
//
//             let mut contract_model: Box<Result<ContractModel, ContractError>> = if let Some(ref curr_terms) = curr_terms {
//                 // Supposons que ContractModel::new retourne Result<ContractModel, String>
//                 match ContractModel::new(&curr_terms) {
//                     Ok(model) => Box::new(Ok(model)),
//                     Err(e) => Box::new(Err(ContractError::from(e))),
//                 }
//             } else {
//                 Box::new(Err(ContractError::MissingTerms))
//             };
//
//             let risk_factor_model = RiskFactorModel;
//
//
//             let mut vec_results: Vec<HashMap<String, Value>> = vec![];
//             if let Ok(cm) = contract_model.as_ref() {
//                 let mut events = Swap::schedule(&to_date.unwrap(), cm); //PrincipalAtMaturity::schedule(&to_date, cm);
//
//                 if let Ok(events_res) = events {
//                     let events2 = Swap::apply(events_res, cm, &risk_factor_model);
//
//                     for ce in events2.iter() {
//                         let mut sub_res_hm: HashMap<String, Value> = HashMap::new();
//                         sub_res_hm.insert("eventDate".to_string(), Value::String( ce.event_time.unwrap().format("%Y-%m-%dT%H:%M:%S").to_string() ));
//                         sub_res_hm.insert("eventType".to_string(), Value::String( ce.event_type.clone().to_string() ));
//                         sub_res_hm.insert("payoff".to_string(), Value::F64( ce.payoff.unwrap() ));
//                         sub_res_hm.insert("currency".to_string(),Value::String( ce.currency.clone().unwrap() ));
//                         let ci = ce.contract_id.clone().unwrap();
//
//                         let vContRef: Vec<ContractReference> = cm.contractStructure.clone()
//                             .unwrap_or_default()
//                             .iter()
//                             .filter(|cr| {
//                                 cr.object.as_cm()
//                                     .and_then(|cm_obj| cm_obj.contract_id.clone())
//                                     .map_or(false, |id| id == ci)
//                             })
//                             .cloned()
//                             .collect();
//
//                         let not2 = vContRef.get(0)
//                             .and_then(|cr| cr.object.as_cm())
//                             .and_then(|cm_obj| cm_obj.notionalPrincipal.clone())
//                             .unwrap_or_default();
//
//                         sub_res_hm.insert("notionalPrincipal".to_string(), Value::F64( not2 ));
//
//                         let nomInt2 = vContRef.get(0)
//                             .and_then(|cr| cr.object.as_cm())
//                             .and_then(|cm_obj| cm_obj.nominalInterestRate.clone())
//                             .unwrap_or_default();
//                         sub_res_hm.insert("nominalInterestRate".to_string(), Value::F64( nomInt2  ));
//
//                         let accInt = vContRef.get(0)
//                             .and_then(|cr| cr.object.as_cm())
//                             .and_then(|cm_obj| cm_obj.accruedInterest.clone())
//                             .unwrap_or_default();
//                         sub_res_hm.insert("accruedInterest".to_string(), Value::F64( accInt  ));
//
//                         println!("EventTime: {:?} - EventType: {:?} - Payoff: {:?} - State.AccruedInterest: {:?}\n", ce.event_time.unwrap(), ce.event_type, ce.payoff, ce.state.accruedInterest);
//                         vec_results.push(sub_res_hm);
//                     }
//
//
//                     println!("events");
//                 }
//             }
//              // enlever pour faire les autres tests
//
//             let mut v1  = convert_value_map_to_string_map_ref(&vec_results);
//             let mut v2  = convert_value_map_to_string_map_ref(&curr_results.clone());
//             let t = are_contracts_equal(vec_results, curr_results.clone());
//             //assert!(are_contracts_equal(&vec_results, &curr_results));
//             println!("events");
//             break;
//
//         }
//
//     }
// }
