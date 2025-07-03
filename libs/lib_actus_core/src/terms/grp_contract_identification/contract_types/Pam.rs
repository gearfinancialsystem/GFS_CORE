use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;

use crate::functions::pam::pof::{
    POF_FP_PAM::POF_FP_PAM,
    POF_IED_PAM::POF_IED_PAM,
    POF_IP_PAM::POF_IP_PAM,
    POF_IPCI_PAM::POF_IPCI_PAM,
    POF_MD_PAM::POF_MD_PAM,
    POF_PRD_PAM::POF_PRD_PAM,
    POF_RR_PAM::POF_RR_PAM,
    POF_SC_PAM::POF_SC_PAM,
    POF_TD_PAM::POF_TD_PAM
};

use crate::functions::pam::stf::{
    STF_FP_PAM::STF_FP_PAM,
    STF_IED_PAM::STF_IED_PAM,
    STF_IP_PAM::STF_IP_PAM,
    STF_IPCI_PAM::STF_IPCI_PAM,
    STF_MD_PAM::STF_MD_PAM,
    STF_PRD_PAM::STF_PRD_PAM,
    STF_RR_PAM::STF_RR_PAM,
    STF_RRF_PAM::STF_RRF_PAM,
    STF_SC_PAM::STF_SC_PAM,
    STF_TD_PAM::STF_TD_PAM,
};

use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::events::AnyContractEvent::AnyContractEvent;
use crate::terms::grp_dividend::CycleAnchorDateOfDividend::CycleAnchorDateOfDividend;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;

/// Represents the Principal At Maturity payoff algorithm
pub struct PAM;

impl PAM {
    /// Compute next events within the period up to `to` date based on the contract model
    pub fn schedule(to: &IsoDatetime, model: &ContractModel) 
    -> Result<Vec<AnyContractEvent>, Box<dyn Error>> {
        let mut events: Vec<AnyContractEvent> = Vec::new();

        // Initial exchange (IED)
        let e = EventFactory::<InitialExchangeDate, InitialExchangeDate>::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_PAM)),
            &model.contract_id,
        );

        events.push(AnyContractEvent::from_contract_event(e).unwrap());

        // Principal redemption (MD)

        let a = EventFactory::<MaturityDate, MaturityDate>::create_event(
            &Some(model.maturity_date.clone().unwrap().deref().clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_PAM)),
            &model.contract_id,
        );

        events.push(AnyContractEvent::from_contract_event(a).unwrap());

        // Purchase (PRD)
        if model.purchase_date.is_some() {
            let z = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_PAM)),
                Some(Rc::new(STF_PRD_PAM)),
                &model.contract_id,
            );

            events.push(AnyContractEvent::from_contract_event(z).unwrap());
        }

        // Interest payment related events
        if model.nominal_interest_rate.is_some()
            && (model.cycle_of_interest_payment.is_some() || model.cycle_anchor_date_of_interest_payment.is_some())
        {
            // Generate raw interest payment events (IP)
            //let a = model.cycle_anchor_date_of_Interest_payment.clone().unwrap().format("%Y-%m-%d").to_string();
            //let b = model.maturity_date.clone().unwrap().format("%Y-%m-%d").to_string();
            let z = &ScheduleFactory::<
                CycleAnchorDateOfInterestPayment, 
                MaturityDate,
                CycleOfInterestPayment,
                IsoDatetime
            >::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_interest_payment.clone(),
                &model.end_of_month_convention.unwrap(),
                true,
            );
            //let zz = z.iter().map(|a| a.format("%Y-%m-%d").to_string()).collect::<Vec<String>>();
            let mut interest_events = EventFactory::create_events_with_convention(
                z,
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_PAM)),
                Some(Rc::new(STF_IP_PAM)),
                model.business_day_adjuster.as_ref().unwrap(),
                &model.contract_id,
            );

            // Adapt if interest capitalization is set
            if model.capitalization_end_date.is_some() {
                // Remove IP events at IPCED and add IPCI event instead
                let capitalization_end = EventFactory::create_event_with_convention(
                    &model.capitalization_end_date,
                    &EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(Rc::new(STF_IPCI_PAM)),
                    &model.business_day_adjuster.as_ref().unwrap(),
                    &model.contract_id,
                );

                // Remove IP events that occur at capitalization end date
                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && e.event_time == Some(capitalization_end.get_event_time()))
                });

                // Add capitalization end event
                interest_events.insert(capitalization_end.clone());
                let mut vec: Vec<_> = interest_events.clone().into_iter().collect();
                // Change events with time <= IPCED and cont_type IP to IPCI


                vec.iter_mut()
                    .filter(|e| e.event_type == EventType::IP &&
                        e.get_event_time() <= capitalization_end.get_event_time())
                    .for_each(|e| {
                        e.chg_eventType(EventType::IPCI);
                        e.set_f_pay_off(Some(Rc::new(POF_IPCI_PAM)));
                        e.set_f_state_trans(Some(Rc::new(STF_IPCI_PAM)));
                    });

                // for e in vec.iter_mut() {
                //     if e.get_eventType() == EventType::IP
                //         && e.get_event_time() <= capitalization_end.get_event_time()
                //     {
                //         e.chg_eventType(EventType::IPCI);
                //         e.set_f_pay_off(Some(Rc::new(POF_IPCI_PAM)));
                //         e.set_f_state_trans(Some(Rc::new(STF_IPCI_PAM)));
                //     }
                // }
                interest_events = vec.into_iter().collect();
            }

            events.extend(interest_events);
        }
        else if model.capitalization_end_date.is_some() {
            // If no interest schedule set but capitalization end date, add single IPCI event
            events.push(EventFactory::create_event_with_convention(
                &model.capitalization_end_date,
                EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(Rc::new(STF_IPCI_PAM)),
                &model.business_day_adjuster.clone().unwrap(),
                &model.contract_id,
            ));
        }

        // Rate reset events (RR)
        let mut rate_reset_events = EventFactory::create_events_with_convention(
            &ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_rate_reset.clone(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_rate_reset.clone(),
                &model.end_of_month_convention.clone().unwrap(),
                false,
            ),
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            &model.business_day_adjuster.clone().unwrap(),
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::create_event(
                model.status_date,
                EventType::AD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );
            let mut vec: Vec<_> = rate_reset_events.clone().into_iter().collect();
            vec.sort();
            let fixed_event = vec.iter_mut().filter(|e| e.compare_to(&status_event)  == 1 ).next();

            if let Some(fixed_event_val) = fixed_event {
                fixed_event_val.set_f_state_trans(Some(Rc::new(STF_RRF_PAM)));
                fixed_event_val.chg_eventType(EventType::RRF);
                rate_reset_events.insert(fixed_event_val.clone());
            }


        }

        // Add all rate reset events
        events.extend(rate_reset_events);

        // Fee payment events (FP), if specified
        if model.cycle_of_fee.is_some() {
            let fee_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_fee,
                    model.maturity_date.clone().map(|rc| (*rc).clone()),
                    model.cycle_of_fee.clone(),
                    model.end_of_month_convention.unwrap(),
                    true,
                ),
                EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_PAM)),
                &model.business_day_adjuster.clone().unwrap(),
                &model.contract_id,
            );
            events.extend(fee_events);
        }

        // Scaling events (SC), if specified

        if model.scaling_effect.is_some() && (model.scaling_effect.clone().unwrap().to_string().contains('I') || model.scaling_effect.clone().unwrap().to_string().contains('N'))
        {
            let scaling_events = EventFactory::create_events_with_convention(
                &ScheduleFactory::create_schedule(
                    model.cycle_anchor_date_of_scaling_index,
                    model.maturity_date.clone().map(|rc| (*rc).clone()),
                    model.cycle_of_scaling_index.clone(),
                    model.end_of_month_convention.unwrap(),
                    false,
                ),
                EventType::SC,
                &model.currency,
                Some(Rc::new(POF_SC_PAM)),
                Some(Rc::new(STF_SC_PAM)),
                &model.business_day_adjuster.clone().unwrap(),
                &model.contract_id,
            );
            events.extend(scaling_events);
        }

        // Termination event (TD)
        if model.termination_date.is_some() {
            let termination = EventFactory::create_event(
                model.termination_date,
                EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_PAM)),
                Some(Rc::new(STF_TD_PAM)),
                &model.contract_id,
            );

            // Remove all events occurring after termination date
            events.retain(|e| e <= &termination);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_date = model.status_date;
        let status_event = EventFactory::create_event(
            status_date,
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );
        events.retain(|e| e >= &status_event);

        // Remove all events after the `to` date
        let to_event = EventFactory::create_event(
            Some(to.clone()),
            EventType::AD,
            &model.currency,
            None,
            None,
            &model.contract_id,
        );
        events.retain(|e| e <= &to_event);

        // Sort events according to their time of occurrence
        events.sort();

        Ok(events.clone())
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    pub fn apply(events: Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel)
     -> Vec<ContractEvent> {
        // Initialize state space per status date
        let mut states = Self::init_StateSpace(model);
        let mut events = events.clone();
        // Sort events according to their time sequence
        events.sort();
        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.clone().unwrap(),
                &model.business_day_adjuster.clone().unwrap(),
            );
        }

        // Remove pre-purchase events if purchase date is set
        if model.purchase_date.is_some() {
            let purchase_date = model.purchase_date;
            let purchase_event = EventFactory::create_event(
                purchase_date,
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &model.contract_id,
            );
            events.retain(|e| {
                e.get_eventType() == EventType::AD || e >= &purchase_event
            });
        }

        // Return evaluated events
        events.clone()
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_StateSpace(model: &ContractModel) 
    -> StateSpace {
        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = model.interest_scaling_multiplier.clone();
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date.clone();

        let initial_exchange_date: IsoDatetime = model.initial_exchange_date.clone().unwrap();
        if initial_exchange_date > states.status_date.unwrap() {
            states.notional_principal = Some(0.0);
            states.nominal_interest_rate = Some(0.0);
        } else {

            let role_sign = model.contract_role.as_ref().map_or(1.0, |a| a.role_sign());
            states.notional_principal = Some(role_sign * model.notional_principal.unwrap());
            states.nominal_interest_rate = model.nominal_interest_rate;
        }

        // Initialize accrued interest
        if model.nominal_interest_rate.is_none() {
            states.accrued_interest =  Some(0.0);
        } else if model.accrued_interest.is_some() {
            states.accrued_interest = model.accrued_interest;
        } else {
            // GERER CE CAS : Il y a UNE ERREUR
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.business_day_adjuster.as_ref().unwrap();


            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment,
                model.maturity_date.clone().map(|rc| (*rc).clone()),
                model.cycle_of_interest_payment.clone(),
                model.end_of_month_convention.unwrap(),
                true,
            ).into_iter().collect();

            ip_schedule.sort();

            let date_earlier_than_t0: Vec<&IsoDatetime> = ip_schedule
                .iter()
                .filter(|&&date| date < states.status_date.unwrap())
                .collect();

            let t_minus = date_earlier_than_t0.last();
            println!("ok");
            states.accrued_interest = Some(day_counter.day_count_fraction(time_adjuster.shift_bd(t_minus.unwrap()),
                                                                         time_adjuster.shift_bd(&states.status_date.unwrap()))
                * states.notional_principal.unwrap()
                * states.nominal_interest_rate.unwrap());

        }

        if model.fee_rate.is_none() {
            states.fee_accrued = Some(0.0);
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued;
        }
        // TODO: Implement last two possible initializations if needed

        states
    }
}
impl fmt::Display for PAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PAM")
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::{Value as ValueS, Map};
//     use std::fs::File;
//     use std::io::Read;
//     use std::error::Error;
//     use std::collections::HashMap;
//     use crate::exceptions::ContractTypeUnknownException::ContractError;
//     use c>rate::util::Value::Value;
//     use crate::util_tests::TestsUtils::test_read_and_parse_json;
//     use crate::util_tests::TestsUtils::json_to_dico;

//     fn load_dico_tests() -> Vec<Value> {
//         let pathx = "/home/cet/Projects/ACTUS-CORE/actus-core-master-rust-project-v2/libs/lib_actus_core/tests_sets/actus-tests-pam.json";
//         let json_value = test_read_and_parse_json(pathx).unwrap();
//         let dico_from_json = json_to_dico(json_value);
//         dico_from_json
//     }

//     #[test]
//     fn test_pam_contracts(){
//         let dico_tests = load_dico_tests();

//         //let dico_tests: Vec<HashMap<String, Value>> = vec![load_dico_tests()];
//         for el in dico_tests.iter() {

//             let curr_test = el.as_hashmap().unwrap();

//             let curr_identifier = curr_test.get("identifier").unwrap().as_string();
//             let curr_terms = curr_test.get("terms").unwrap().as_hashmap();
//             let curr_to = curr_test.get("to").unwrap().as_string();
//             let curr_data_observed = curr_test.get("dataObserved").unwrap().as_hashmap(); // verifier si cest None
//             let curr_events_observed = curr_test.get("eventsObserved").unwrap().as_vec();
//             let curr_results = curr_test.get("results").unwrap().as_vec().unwrap();
//             //let a = curr_results.get(0).unwrap().get("notionalPrincipal").unwrap().as_string().unwrap();
//             let to_date = if let Some(curr_to) = curr_to {
//                 IsoDatetime::parse_from_str(&curr_to, "%Y-%m-%dT%H:%M:%S").ok()
//             } else {
//                 None
//             };

//             let mut contract_model: Box<Result<ContractModel, ContractError>> = if let Some(ref curr_terms) = curr_terms {
//                 // Supposons que ContractModel::new retourne Result<ContractModel, String>
//                 match ContractModel::new(&curr_terms) {
//                     Ok(model) => Box::new(Ok(model)),
//                     Err(e) => Box::new(Err(ContractError::from(e))),
//                 }
//             } else {
//                 Box::new(Err(ContractError::MissingTerms))
//             };

//             let risk_factor_model = RiskFactorModel;


//             let mut vec_results: Vec<HashMap<String, Value>> = vec![];
//         }
//         true
//     }
// }