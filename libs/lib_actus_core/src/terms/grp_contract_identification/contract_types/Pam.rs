
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use crate::events::ContractEvent::{ContractEvent};
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
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
//use crate::events::AnyContractEvent::AnyContractEvent;

use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::traits::TraitContractModel::TraitContractModel;

/// Represents the Principal At Maturity payoff algorithm
pub struct PAM;

impl TraitContractModel for PAM {
    /// Compute next events within the period up to `to` date based on the contract model
    fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        
        //let mut events: Vec<Box< dyn TraitContractEvent>> = Vec::new();
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let maturity_date = model.maturity_date.clone().unwrap().deref().clone();

        ////////////////////////////
        // Initial exchange (IED) //
        ////////////////////////////
        let e = EventFactory::<InitialExchangeDate, InitialExchangeDate>::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_PAM)),
            &None,
            &model.contract_id);

        events.push(e.to_iso_datetime_event());

        ///////////////////////////////
        // Principal redemption (MD) //
        /////////////////////////////// 
        let e = EventFactory::<MaturityDate, MaturityDate>::create_event(
            &Some(maturity_date.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_PAM)),
            &None,
            &model.contract_id);
        events.push(e.to_iso_datetime_event());

        ///////////////////////////////
        //       Purchase (PRD)      //
        ///////////////////////////////
        //let aa = model.purchase_date.is_some();
        if model.purchase_date.is_some() {
            //let a = false;
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_PAM)),
                Some(Rc::new(STF_PRD_PAM)),
                &None,
                &model.contract_id);
            events.push(e.to_iso_datetime_event());
        }
        
        /////////////////////////////////////
        // Interest payment related events //
        /////////////////////////////////////
        if model.nominal_interest_rate.is_some() && 
            (model.cycle_of_interest_payment.is_some() || 
            model.cycle_anchor_date_of_interest_payment.is_some()){

            // Generate raw interest payment events (IP)
            let z = &ScheduleFactory::
                <CycleAnchorDateOfInterestPayment, 
                MaturityDate, 
                CycleOfInterestPayment,
                IsoDatetime>::create_schedule(  
                    &model.cycle_anchor_date_of_interest_payment,
                    &Some(maturity_date.clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(true));

            let mut interest_events = EventFactory::create_events(
                z,
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_PAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id);

            // Adapt if interest capitalization is set
            if model.capitalization_end_date.is_some() {
                // Remove IP events at IPCED and add IPCI event instead
                let a = model.capitalization_end_date.clone().unwrap();
                let b : Option<IsoDatetime> = a.try_into().ok();
                let c : IsoDatetime = IsoDatetime::new(b.unwrap().date(), b.unwrap().time());

                let capitalization_end = EventFactory::create_event(
                    &Some(c),
                    &EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(Rc::new(STF_IPCI_PAM)),
                    &model.business_day_adjuster,
                    &model.contract_id);

                // Remove IP events that occur at capitalization end date
                interest_events.retain(|e| {
                    !(e.event_type != EventType::IP || e.event_time != Some(capitalization_end.get_event_time()))
                }); // A REVOIR

                // Add capitalization end event
                interest_events.insert(capitalization_end.clone() );
                let mut vec: Vec<_> = interest_events.clone().into_iter().collect();
                // Change events with time <= IPCED and cont_type IP to IPCI


                vec.iter_mut()
                    .filter(|e| e.event_type == EventType::IP &&
                        e.get_event_time() <= capitalization_end.get_event_time())
                    .for_each(|e| {
                        e.chg_event_type(EventType::IPCI);
                        e.set_f_pay_off(Some(Rc::new(POF_IPCI_PAM)));
                        e.set_f_state_trans(Some(Rc::new(STF_IPCI_PAM)));
                    });

                // interest_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = vec.into_iter().collect();
                
            }
            let w: Vec<Box<ContractEvent<IsoDatetime, IsoDatetime>>> = interest_events.into_iter().map(|ce| Box::new(ce)).collect();
            for el in w.into_iter(){
                events.push(el.to_iso_datetime_event());
            }
            
            //events.extend(w);
        }
        else if model.capitalization_end_date.is_some() {
            // If no interest schedule set but capitalization end date, add single IPCI event
            let a: ContractEvent<CapitalizationEndDate, CapitalizationEndDate> = EventFactory::create_event( // lannotation est peut etre fausse a verifier
                &model.capitalization_end_date,
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(Rc::new(STF_IPCI_PAM)),
                &model.business_day_adjuster,
                &model.contract_id);

            events.push(a.to_iso_datetime_event());
        }

        ////////////////////////////
        // Rate reset events (RR) //
        ////////////////////////////
        let a = &ScheduleFactory::
                <CycleAnchorDateOfRateReset, 
                MaturityDate,
                CycleOfRateReset,
                IsoDatetime>::create_schedule(
                &model.cycle_anchor_date_of_rate_reset,
                &Some(maturity_date),
                &model.cycle_of_rate_reset,
                &model.end_of_month_convention,
                Some(false),
            );
        
        let mut rate_reset_events = EventFactory::create_events(
            a,
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::<StatusDate, StatusDate>::create_event(
                &model.status_date,
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );
            let mut vec: Vec<_> = rate_reset_events.clone().into_iter().collect();
            vec.sort();
            let fixed_event = vec.iter_mut().filter(|e| e.compare_to(&status_event.to_iso_datetime_event())  == 1 ).next();

            if let Some(fixed_event_val) = fixed_event {
                fixed_event_val.set_f_state_trans(Some(Rc::new(STF_RRF_PAM)));
                fixed_event_val.chg_event_type(EventType::RRF);
                rate_reset_events.insert(fixed_event_val.clone());
            }
        }

        // Add all rate reset events
        //events.extend(rate_reset_events);

        let w: Vec<Box<ContractEvent<IsoDatetime, IsoDatetime>>> = rate_reset_events.into_iter().map(|ce| Box::new(ce)).collect();
        for el in w.into_iter(){
            events.push(el.to_iso_datetime_event());
        }

        ///////////////////////////////////////////
        // Fee payment events (FP), if specified //
        ///////////////////////////////////////////
        if model.cycle_of_fee.is_some() {
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &model.cycle_of_fee.clone(),
                    &model.end_of_month_convention,
                    Some(true),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(fee_events);
        }
        
        ///////////////////////////////////////
        // Scaling events (SC), if specified //
        ///////////////////////////////////////
        if model.scaling_effect.is_some() && 
            (model.scaling_effect.clone().unwrap().to_string().contains('I') || 
             model.scaling_effect.clone().unwrap().to_string().contains('N'))
        {
            let scaling_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_scaling_index,
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &model.cycle_of_scaling_index.clone(),
                    &model.end_of_month_convention,
                    Some(false),
                ),
                &EventType::SC,
                &model.currency,
                Some(Rc::new(POF_SC_PAM)),
                Some(Rc::new(STF_SC_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(scaling_events);
        }

        ////////////////////////////
        // Termination event (TD) //
        ////////////////////////////
        if model.termination_date.is_some() {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &model.termination_date,
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_PAM)),
                Some(Rc::new(STF_TD_PAM)),
                &None,
                &model.contract_id,
            );

            // Remove all events occurring after termination date
            events.retain(|e| e <= &termination.to_iso_datetime_event());
            events.push(termination.to_iso_datetime_event());
        }

        
        ///////////////////////////////////////
        // Remove all pre-status date events //
        ///////////////////////////////////////
        let status_date = model.status_date.clone().unwrap();
        let status_event : ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &Some(status_date),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id);
        events.retain(|e| e >= &status_event.to_iso_datetime_event());

        ///////////////////////////////////////////
        // Remove all events after the `to` date //
        ///////////////////////////////////////////
        let to_event: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );
        events.retain(|e| e <= &to_event.to_iso_datetime_event());

        ///////////////////////////////////////////////////////
        // Sort events according to their time of occurrence //
        ///////////////////////////////////////////////////////
        events.sort();

        Ok(events.clone())
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>, model: &ContractModel, observer: &RiskFactorModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {
        ////////////////////////////////////////////
        // Initialize state space per status date //
        ////////////////////////////////////////////
        let _maturity = &model.maturity_date.clone().unwrap().clone();
        let mut states = Self::init_state_space(model, observer, _maturity).expect("uncorrect state space initialization !");
        let mut events = events.clone();

        //////////////////////////////////////////////////
        // Sort events according to their time sequence //
        //////////////////////////////////////////////////
        events.sort();

        ////////////////////////////////////////////////////////////////////
        // Apply events according to their time sequence to current state //
        ////////////////////////////////////////////////////////////////////
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                &model.day_count_convention.clone().unwrap(),
                &model.business_day_adjuster.clone().unwrap(),
            );
        }

        ////////////////////////////////////////////////////////
        // Remove pre-purchase events if purchase date is set //
        ////////////////////////////////////////////////////////
        if model.purchase_date.is_some() {
            // let purchase_date = model.purchase_date;
            let purchase_event: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );
            events.retain(|e| {
                e.get_event_type() == EventType::AD || e >= &purchase_event.to_iso_datetime_event()
            });
        }
        /////////////////////////////
        // Return evaluated events //
        /////////////////////////////
        Ok(events)
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_state_space(model: &ContractModel, _observer: &RiskFactorModel, _maturity: &MaturityDate) -> Result<StateSpace, String> {
        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = model.interest_scaling_multiplier.clone();
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date.clone();

        let initial_exchange_date: IsoDatetime = model.initial_exchange_date.clone().unwrap().value();
        if initial_exchange_date > states.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok();
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok()
        } else {

            let role_sign = model.contract_role.as_ref().map_or(1.0, |a| a.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
        }

        // Initialize accrued interest
        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = AccruedInterest::new(0.0).ok();
        } else if model.accrued_interest.is_some() {
            states.accrued_interest = model.accrued_interest.clone();
        } else {
            // GERER CE CAS : Il y a UNE ERREUR
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.business_day_adjuster.as_ref().unwrap();


            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment,
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_interest_payment.clone(),
                &model.end_of_month_convention,
                Some(true),
            ).into_iter().collect();

            ip_schedule.sort();

            let sd = states.status_date.clone().unwrap().value();
            let date_earlier_than_t0: Vec<IsoDatetime> = ip_schedule
                .into_iter()
                .filter(|&date| date < sd )
                .collect();

            let t_minus = date_earlier_than_t0.last();

            states.accrued_interest = AccruedInterest::new(
                day_counter.day_count_fraction(
                    time_adjuster.shift_bd(t_minus.unwrap()),
                    time_adjuster.shift_bd(&states.status_date.clone().unwrap().value()))
                * states.notional_principal.clone().unwrap().value()
                * states.nominal_interest_rate.clone().unwrap().value()
                ).ok()
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued.clone();
        }
        // TODO: Implement last two possible initializations if needed

        Ok(states)
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