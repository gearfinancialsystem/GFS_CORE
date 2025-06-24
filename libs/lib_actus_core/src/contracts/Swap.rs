use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use log::debug;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_PRD_PAM::POF_PRD_PAM;
use crate::functions::swaps::pof::POF_NET_SWAPS::POF_NET_SWAPS;
use crate::functions::swaps::pof::POF_PRD_SWAPS::POF_PRD_SWAPS;
use crate::functions::swaps::pof::POF_TD_SWAPS::POF_TD_SWAPS;
use crate::functions::swaps::stf::STF_NET_SWAPS::STF_NET_SWAPS;
use crate::functions::swaps::stf::STF_PRD_SWAPS::STF_PRD_SWAPS;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::state_space::StateSpace::StateSpace;
use crate::util::CommonUtils::CommonUtils;

use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::S::S;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::{traitNaiveDateTimeExtension, IsoDatetime};


// use crate::contracts::PrincipalAtMaturity::PrincipalAtMaturity;
pub struct Swap;

impl Swap {
    /// Compute next events within the period up to `to` date based on the contract model
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events: Vec<ContractEvent> = vec![];
        let first_leg_model = model.clone().contractStructure.unwrap().iter().filter(|cr| cr.reference_role == ReferenceRole::FIL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        let second_leg_model = model.clone().contractStructure.unwrap().iter().filter(|cr| cr.reference_role == ReferenceRole::SEL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        let mut first_leg_schedule: Vec<ContractEvent> = vec![];
        let mut second_leg_schedule: Vec<ContractEvent> = vec![];

        let mat1 = first_leg_model.maturityDate.clone().map(|rc| (*rc).clone());
        let mat2 = second_leg_model.maturityDate.clone().map(|rc| (*rc).clone());

        first_leg_schedule = ContractType::schedule(mat1,&first_leg_model).unwrap();
        second_leg_schedule = ContractType::schedule(mat2, &second_leg_model).unwrap();
        events.extend(first_leg_schedule);
        events.extend(second_leg_schedule);

        if model.purchaseDate.is_some() {
            events.push(
                EventFactory::create_event(model.purchaseDate,
                                          EventType::PRD,
                                          model.currency.as_ref(),
                                          Some(Rc::new(POF_PRD_SWAPS)),
                                          Some(Rc::new(STF_PRD_SWAPS)),
                                          model.contractID.as_ref())
            )
        }

        if model.terminationDate.is_some() {
            let termination = EventFactory::create_event(
                model.terminationDate.clone(),
                EventType::TD,
                model.currency.as_ref(),
                Some(Rc::new(POF_TD_SWAPS)),
                Some(Rc::new(STF_TD_STK)),
                model.contractID.as_ref()
            );
            events.retain(|e| e.compare_to(&termination) == 1);
            events.push(termination);
        }
        events.retain(|e| e.compare_to(
            &EventFactory::create_event(
                model.statusDate.clone(),
                EventType::AD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref()) ) != -1);
        events.retain(|e| e.compare_to(
            &EventFactory::create_event(
                Some(to.clone()),
                EventType::AD,
                model.currency.as_ref(),
                None,
                None,
                model.contractID.as_ref()) ) != 1);
        Ok(events)
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    pub fn apply(
        events: Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        
        let mut events = events;
        
        let first_leg_model = model.clone().contractStructure.unwrap().iter().filter(|cr| cr.reference_role == ReferenceRole::FIL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        let second_leg_model = model.clone().contractStructure.unwrap().iter().filter(|cr| cr.reference_role == ReferenceRole::SEL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();

        let first_leg_schedule: Vec<ContractEvent> = events
            .iter()
            .filter(|event| {
                first_leg_model.clone().contractID.unwrap_or_default() == event.get_contractID()
            })
            .cloned()
            .collect();

        let second_leg_schedule: Vec<ContractEvent> = events.clone()
            .iter()
            .filter(|event| {
                second_leg_model.clone().contractID.unwrap_or_default() == event.get_contractID()
            })
            .cloned()
            .collect();


        // Remove the filtered events from the main events list
        events.retain(|e| {
            !first_leg_schedule.iter().any(|first_leg_event| first_leg_event.contractID == e.contractID) &&
                !second_leg_schedule.iter().any(|second_leg_event| second_leg_event.contractID == e.contractID)
        });
        
        
        let first_leg_events = ContractType::apply(first_leg_schedule, &first_leg_model.clone(), observer);
        let second_leg_events = ContractType::apply(second_leg_schedule, &second_leg_model.clone(), observer);

        if model.deliverySettlement.clone().unwrap() == DeliverySettlement::S(S) {
            let a = Swap::filter_and_nett_congruent_events(
                first_leg_events.clone().unwrap(),
                second_leg_events.clone().unwrap(),
                model.contractID.clone().unwrap()
            );
            events.extend(a);
        } else {
            events.extend(first_leg_events.clone().unwrap());
            events.extend(second_leg_events.clone().unwrap());
        }

        events.iter().for_each(|event| {
            if event.get_contractID() == model.contractID.clone().unwrap() {
                if event.eventType == EventType::PRD || event.eventType == EventType::TD {
                    let mut parent_state = StateSpace::default();
                    let f_l_events_at_timepoint = first_leg_events.clone().unwrap().iter().filter(|e| {
                        e.eventTime == event.eventTime
                    }).map(|e| e.clone()).collect::<Vec<_>>();
                    let s_l_events_at_timepoint = second_leg_events.clone().unwrap().iter().filter(|e| {
                        e.eventTime == event.eventTime
                    }).map(|e| e.clone()).collect::<Vec<_>>();

                    let mut fl_ipac: f64;
                    let mut sl_ipac: f64;
                    if f_l_events_at_timepoint.is_empty() {
                        fl_ipac = 0.0;
                    }
                    else {
                        fl_ipac = if f_l_events_at_timepoint.iter().any(|e| e.eventType == EventType::IP) {
                            0.0
                        } else {
                            f_l_events_at_timepoint.iter()
                                .find(|e| e.eventType == EventType::PR)
                                .map(|e| e.states().accruedInterest.clone().unwrap())
                                .unwrap_or(0.0)
                        };
                    }
                    sl_ipac = if s_l_events_at_timepoint.is_empty() {
                            0.0
                        } else {
                            if s_l_events_at_timepoint.iter().any(|e| e.eventType == EventType::IP) {
                                0.0
                            } else {
                                s_l_events_at_timepoint.iter()
                                    .find(|e| e.eventType == EventType::PR)
                                    .map(|e| e.states().accruedInterest.clone().unwrap())
                                    .unwrap_or(0.0)
                            }
                        };
                    parent_state.accruedInterest = Some(fl_ipac + sl_ipac);

                }
                else {
                    //event.clone().eval(None, None, None, None, None);
                    //A REFLECHIR
                }
            }
        });
        if model.purchaseDate.is_some(){
            let purchase = EventFactory::create_event(
              model.purchaseDate.clone(),
              EventType::PRD,
              model.currency.as_ref(),
              Some(Rc::new(POF_PRD_SWAPS)),
              Some(Rc::new(STF_PRD_STK)), // WHY ?
              model.contractID.as_ref()
            );
            // Remove the filtered events from the main events list
            events.retain(|e| {
                e.compare_to(&purchase) == -1
            });
        }
        events.sort();
        events
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_StateSpace(
        model: &ContractModel,
        event_at_t0: ContractEvent
    ) -> StateSpace {

        let first_leg_model = model.clone().contractStructure.unwrap().iter().filter(|cr| cr.reference_role == ReferenceRole::FIL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();
        let second_leg_model = model.clone().contractStructure.unwrap().iter().filter(|cr| cr.reference_role == ReferenceRole::SEL).map(|cr| cr.clone().object).collect::<Vec<_>>().get(0).unwrap().clone().as_cm().unwrap();

        let event_t0_status_date = event_at_t0.states().statusDate;
        let mut states = if event_t0_status_date.is_some() {
            StateSpace::default()
        } else { event_at_t0.states() };

        states.statusDate = model.statusDate.clone();
        states.contractPerformance = if model.contractPerformance.is_some() {
            model.contractPerformance
        } else { None };
        let mat1 = first_leg_model.maturityDate.clone().map(|rc| (*rc).clone());
        let mat2 = second_leg_model.maturityDate.clone().map(|rc| (*rc).clone());
        states.maturityDate = if mat1 > mat2 { mat1 } else { mat2 };
        states.accruedInterest = event_at_t0.states().accruedInterest;
        states
    }

    pub fn filter_and_nett_congruent_events(first_leg_events: Vec<ContractEvent>, second_leg_events: Vec<ContractEvent>, parent_contract_ID: String)-> Vec<ContractEvent> {
        let mut first_leg_events = first_leg_events;
        let mut second_leg_events = second_leg_events;
        first_leg_events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));
        second_leg_events.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

        let mut events = Vec::new();

        // Helper function to filter events by type
        let filter_events = |events: &[ContractEvent], event_type: EventType| -> Vec<ContractEvent> {
            events.iter()
                .filter(|event| event.eventType == event_type)
                .cloned()
                .collect()
        };

        // Define a macro to reduce repetition for each event type
        macro_rules! process_event_type {
            ($event_type:expr) => {
                let mut first_leg_x = filter_events(&first_leg_events, $event_type);
                let mut second_leg_x = filter_events(&second_leg_events, $event_type);
                Swap::net_singular_event(
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
        Swap::net_congruent_events(
            first_leg_pr,
            second_leg_pr,
            &mut events,
            Some(parent_contract_ID.clone()),
        );

        // Process IP events
        let first_leg_ip = filter_events(&first_leg_events, EventType::IP);
        let second_leg_ip = filter_events(&second_leg_events, EventType::IP);
        Swap::net_congruent_events(
            first_leg_ip,
            second_leg_ip,
            &mut events,
            Some(parent_contract_ID.clone()),
        );

        events
    }


    pub fn net_singular_event(parent_contract_id: String,
                              events: &mut Vec<ContractEvent>,
                              list_first_leg: &mut Vec<ContractEvent>,
                              list_second_leg: &mut Vec<ContractEvent>){

        if !list_first_leg.is_empty() && !list_second_leg.is_empty() {
            let first_leg_event = &list_first_leg.clone()[0];
            let second_leg_event = &list_second_leg.clone()[0];

            if first_leg_event.eventTime == second_leg_event.eventTime {
                // Remove from events list - but we can't do exact object matching
                // So we'll use a more functional approach: filter events not matching our two
                let mut new_events = Vec::new();
                let mut first_found = false;
                let mut second_found = false;

                for event in events.drain(..) {
                    if !first_found && event.eventTime == first_leg_event.eventTime {
                        first_found = true;
                        continue; // Skip this event (don't add to new_events)
                    }
                    if !second_found && event.eventTime == second_leg_event.eventTime {
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
                let netting_event = Swap::netting_event(
                    Some(first_leg_event.clone()),
                    Some(second_leg_event.clone()),
                    Some(parent_contract_id),
                );
                events.push(netting_event);
            }
        }
    }

    pub fn net_congruent_events(
        first_leg_events: Vec<ContractEvent>,
        second_leg_events: Vec<ContractEvent>,
        events: &mut Vec<ContractEvent>,
        parent_contract_ID: Option<String>) {

        let mut first_leg = first_leg_events;
        let mut second_leg = second_leg_events;

        // Sort both lists by event time
        first_leg.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));
        second_leg.sort_by(|a, b| a.eventTime.cmp(&b.eventTime));

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

                if first_event.eventTime == second_event.eventTime {
                    // Found matching events, create a netting event
                    let netting_event = Swap::netting_event(
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
                } else if second_event.eventTime < first_event.eventTime {
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
        e1: Option<ContractEvent>,
        e2: Option<ContractEvent>,
        parent_contract_id: Option<String>,
    ) -> ContractEvent {
        let netting = EventFactory::create_event(
            e1.clone().unwrap().eventTime,
            e1.clone().unwrap().eventType,
            e1.clone().unwrap().currency.as_ref(),
            Some(Rc::new(POF_NET_SWAPS::new(e1.clone().unwrap(), e2.clone().unwrap()))),
            Some(Rc::new(STF_NET_SWAPS::new(e1.clone().unwrap(), e2.clone().unwrap()))),
            parent_contract_id.clone().as_ref(),
        );
        netting
    }
}

