use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use crate::external::data_observers::DataObserver1::{DataObserver1, ObservedDataPoint};
use crate::external::event_observers::EventObserver1::EventObserver1;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::types::IsoDatetime::IsoDatetime;

use serde_json::{self, Value as JsonValue};
use crate::util::Value::Value;



pub fn load_events_observed(
    file_path: &str,
    test_case_id: &str,
) -> Result<Vec<EventObserver1>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json: JsonValue = serde_json::from_reader(reader)?;

    let test_case = json.get(test_case_id)
        .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

    let events_observed = test_case.get("eventsObserved")
        .ok_or_else(|| format!("'eventsObserved' section not found in {}", test_case_id))?;

    if let JsonValue::Array(events) = events_observed {
        let mut result = Vec::new();

        for event in events {
            if let JsonValue::Object(event_obj) = event {
                // Extraire les champs de base
                let time = event_obj.get("time")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing time in observed event")?
                    .to_string();

                let typex = event_obj.get("type")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing type in observed event")?
                    .to_string();

                let value = event_obj.get("value")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing or invalid value in observed event")?;

                let contract_id = event_obj.get("contractId")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing contractId in observed event")?
                    .to_string();

                // Parser le StateSpace
                let states_json = event_obj.get("states")
                    .ok_or("Missing states in observed event")?;

                let states = parse_state_space(states_json)?;

                // Créer l'événement observé
                result.push(EventObserver1::new(
                    time,
                    typex,
                    value,
                    contract_id,
                    states
                ));
            } else {
                return Err("Invalid event format".into());
            }
        }

        Ok(result)
    } else {
        Err("eventsObserved should be an array".into())
    }
}

// Fonction pour parser le StateSpace à partir du JSON
fn parse_state_space(states_json: &JsonValue) -> Result<StateSpace, Box<dyn std::error::Error>> {
    let mut state_space = StateSpace::default();

    if let JsonValue::Object(states_obj) = states_json {
        for (key, value) in states_obj {
            match key.as_str() {
                "contractPerformance" => {
                    if let JsonValue::String(s) = value {
                        state_space.contract_performance = ContractPerformance::new(s.clone().as_str()).ok();
                    }
                },
                "nonPerformingDate" => {
                    if let JsonValue::String(s) = value {
                        state_space.non_performing_date =  NonPerformingDate::from_str(s.clone().as_str()).ok();
                    }
                },
                "statusDate" => {
                    if let JsonValue::String(s) = value {
                        state_space.status_date = StatusDate::from_str(s.clone().as_str()).ok();
                    }
                },
                "maturityDate" => {
                    if let JsonValue::String(s) = value {
                        state_space.maturity_date = MaturityDate::from_str(s.clone().as_str()).ok();
                    }
                },
                "terminationDate" => {
                    if let JsonValue::String(s) = value {
                        state_space.termination_date = TerminationDate::from_str(s.clone().as_str()).ok();
                    }
                },
                "exerciseDate" => {
                    if let JsonValue::String(s) = value {
                        state_space.exercise_date = ExerciseDate::from_str(s.clone().as_str()).ok();
                    }
                },
                "accruedInterest" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.accrued_interest = AccruedInterest::new(f).ok();
                        }
                    }
                },
                "accruedInterest2" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.accrued_interest2 = AccruedInterest2::new(f).ok();
                        }
                    }
                },
                "exerciseAmount" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.exercise_amount = ExerciseAmount::new(f).ok();
                        }
                    }
                },
                "feeAccrued" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.fee_accrued = FeeAccrued::new(f).ok();
                        }
                    }
                },
                "interestCalculationBaseAmount" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.interest_calculation_base_amount = InterestCalculationBaseAmount::new(f).ok();
                        }
                    }
                },
                "interestScalingMultiplier" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.interest_scaling_multiplier = InterestScalingMultiplier::new(f).ok();
                        }
                    }
                },
                "nextPrincipalRedemptionPayment" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.next_principal_redemption_payment = NextPrincipalRedemptionPayment::new(f).ok();
                        }
                    }
                },
                "nominalInterestRate" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.nominal_interest_rate = NominalInterestRate::new(f).ok();
                        }
                    }
                },
                "nominalInterestRate2" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.nominal_interest_rate2 = NominalInterestRate2::new(f).ok();
                        }
                    }
                },
                "notionalPrincipal" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.notional_principal = NotionalPrincipal::new(f).ok();
                        }
                    }
                },
                "notionalPrincipal2" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.notional_principal2 = NotionalPrincipal2::new(f).ok();
                        }
                    }
                },
                "notionalScalingMultiplier" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.notional_scaling_multiplier = NotionalScalingMultiplier::new(f).ok();
                        }
                    }
                },
                "boundaryCrossedFlag" => {
                    if let JsonValue::Bool(b) = value {
                        state_space.boundary_crossed_flag = BoundaryCrossedFlag::new(*b).ok();
                    }
                },
                "boundaryMonitoringFlag" => {
                    if let JsonValue::Bool(b) = value {
                        state_space.boundary_monitoring_flag = Some(*b);
                    }
                },
                "boundaryLeg1ActiveFlag" => {
                    if let JsonValue::Bool(b) = value {
                        state_space.boundary_leg1_active_flag = Some(*b);
                    }
                },
                "boundaryLeg2ActiveFlag" => {
                    if let JsonValue::Bool(b) = value {
                        state_space.boundary_leg2_active_flag = Some(*b);
                    }
                },
                "lastInterestPeriod" => {
                    if let JsonValue::Number(n) = value {
                        if let Some(f) = n.as_f64() {
                            state_space.last_interest_period = Some(f);
                        }
                    }
                },
                _ => {
                    // Ignorer les champs inconnus ou logguer un avertissement

                }
            }
        }
    } else {
        return Err("states should be an object".into());
    }

    Ok(state_space)
}



// Fonction de conversion
fn convert_json_value(value: &JsonValue) -> Value {
    match value {
        JsonValue::String(s) => Value::Vstring(s.clone()),
        JsonValue::Object(o) => {
            let mut map = HashMap::new();
            for (k, v) in o {
                map.insert(k.clone(), convert_json_value(v));
            }
            Value::VhashMap(map)
        }
        JsonValue::Array(a) => {
            Value::VvecVal(a.iter().map(convert_json_value).collect())
        }
        _ => Value::Vstring(value.to_string()),
    }
}

// Fonction publique pour charger les termes
pub fn load_test_case_terms(
    file_path: &str,
    test_case_id: &str,
) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json: JsonValue = serde_json::from_reader(reader)?;

    let test_case = json.get(test_case_id)
        .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

    let terms = test_case.get("terms")
        .ok_or_else(|| format!("'terms' section not found in {}", test_case_id))?;

    if let JsonValue::Object(terms_obj) = terms {
        let mut result_map = HashMap::new();
        for (key, value) in terms_obj {
            result_map.insert(key.clone(), convert_json_value(value));
        }
        Ok(result_map)
    } else {
        Err("Invalid 'terms' format".into())
    }
}


