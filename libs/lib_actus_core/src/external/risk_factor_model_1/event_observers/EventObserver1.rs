use serde_json::{self, Value as JsonValue};
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
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
use std::str::FromStr;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventSequence::EventSequence;
use crate::events::EventType::EventType;
use crate::external::composantes::ObservedEventPoint::ObservedEventPoint;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(PartialEq, Debug, Clone)]
pub struct EventObserver1 {
    event_serie_brute: Vec<ObservedEventPoint>,
    event_serie: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
}
impl EventObserver1 {
    pub fn new() -> Self {
        Self {
            event_serie_brute: Vec::<ObservedEventPoint>::new(),
            event_serie: Vec::<ContractEvent<IsoDatetime, IsoDatetime>>::new()
        }
    }

    fn convert_vec_observed_event_point(
        events: Vec<ObservedEventPoint>,
        currency: &str
    ) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, Box<dyn std::error::Error>> {

        events.clone().into_iter().map(|obs_event| {
            // Convertir les temps
            let event_time = IsoDatetime::from_str(&obs_event.get_time())?;
            let schedule_time = event_time.clone(); // Même temps pour schedule_time

            // Convertir le type d'événement
            let event_type = EventType::from_str(&obs_event.get_typex())
                .map_err(|_| format!("Unknown event type: {}", obs_event.get_typex()))?;

            // Créer les wrappers nécessaires
            let currency_wrapper = Currency::new(currency.to_string())
                .map_err(|_| format!("Invalid currency: {}", currency))?;

            let contract_id_wrapper = ContractID::new(obs_event.get_contract_id())
                .map_err(|_| format!("Invalid contract ID: {}", obs_event.get_contract_id()))?;

            // Calculer l'epoch offset
            let epoch_offset = event_time.0.and_utc().timestamp_millis() +
                EventSequence::time_offset(&event_type);

            // Créer l'événement de contrat
            let mut event = ContractEvent {
                _marker_t1: PhantomData,
                _marker_t2: PhantomData,
                epoch_offset: Some(epoch_offset),
                fstate: None,
                fpayoff: None,
                event_time: Some(event_time),
                schedule_time: Some(schedule_time),
                event_type,
                currency: Some(currency_wrapper),
                payoff: Some(obs_event.get_value()),
                state: obs_event.get_states().clone(),
                contract_id: Some(contract_id_wrapper),
            };

            Ok(event)
        }).collect()

    }

    pub fn new_from(
        file_path: &str,
        test_case_id: &str,
    ) -> Result<EventObserver1, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let json: JsonValue = serde_json::from_reader(reader)?;

        let test_case = json.get(test_case_id)
            .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

        let events_observed = test_case.get("eventsObserved")
            .ok_or_else(|| format!("'eventsObserved' section not found in {}", test_case_id))?;

        if let JsonValue::Array(events) = events_observed {
            let mut result: Vec<ObservedEventPoint> = Vec::new();

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

                    let states = Self::parse_state_space(states_json)?;

                    // Créer l'événement observé
                    result.push(ObservedEventPoint::new(
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

            Ok(EventObserver1 {
                event_serie_brute: result.clone(),
                event_serie: Self::convert_vec_observed_event_point(result, "USD")? // ATTENTION ICI CORRIGER CURRENCY
            })
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

}




