use serde_json::{self, Value as JsonValue};
use std::fs::File;
use std::io::BufReader;
use crate::types::IsoDatetime::IsoDatetime;
use crate::events::EventType::EventType;
use crate::terms::grp_notional_principal::Currency::Currency;
use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;

#[derive(PartialEq, Debug, Clone)]
pub struct ResultSet {
    values: HashMap<String, String>,
    payoff: Option<f64>,
    currency: Option<Currency>,
    event_date: Option<IsoDatetime>,
    event_type: Option<EventType>,
    accrued_interest: Option<AccruedInterest>,
    accrued_interest2: Option<AccruedInterest2>,
    exercise_amount: Option<ExerciseAmount>,
    exercise_date: Option<ExerciseDate>,
    fee_accrued: Option<FeeAccrued>,
    interest_calculation_base_amount: Option<InterestCalculationBaseAmount>,
    interest_scaling_multiplier: Option<InterestScalingMultiplier>,
    next_principal_redemption_payment: Option<NextPrincipalRedemptionPayment>,
    nominal_interest_rate: Option<NominalInterestRate>,
    nominal_interest_rate2: Option<NominalInterestRate2>,
    notional_principal: Option<NotionalPrincipal>,
    notional_principal2: Option<NotionalPrincipal2>,
    notional_scaling_multiplier: Option<NotionalScalingMultiplier>,
}
impl ResultSet {
    pub fn get_expected_results(&self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        if let Some(ref event_date) = self.event_date {
            attributes.insert("event_date".to_string(), event_date.value().to_string() );
        }
        if let Some(ref event_type) = self.event_type {
            attributes.insert("event_type".to_string(), event_type.to_string() );
        }
        if let Some(ref currency) = self.currency {
            attributes.insert("currency".to_string(), currency.value().to_string() );
        }
        if let Some(payoff) = self.payoff {
            attributes.insert("payoff".to_string(), payoff.to_string() );
        }

        if let Some(v) = self.accrued_interest.clone() {
            attributes.insert("accrued_interest".to_string(), v.to_string() );
        }
        if let Some(v) = self.accrued_interest2.clone() {
            attributes.insert("accrued_interest2".to_string(), v.to_string() );
        }
        if let Some(v) = self.exercise_amount.clone() {
            attributes.insert("exercise_amount".to_string(), v.to_string() );
        }
        if let Some(v) = self.exercise_date.clone() {
            attributes.insert("exercise_date".to_string(), v.to_string() );
        }
        if let Some(v) = self.fee_accrued.clone() {
            attributes.insert("fee_accrued".to_string(), v.to_string() );
        }
        if let Some(v) = self.interest_calculation_base_amount.clone() {
            attributes.insert("interest_calculation_base_amount".to_string(), v.to_string() );
        }
        if let Some(v) = self.interest_scaling_multiplier.clone() {
            attributes.insert("interest_scaling_multiplier".to_string(), v.to_string() );
        }
        if let Some(v) = self.next_principal_redemption_payment.clone() {
            attributes.insert("next_principal_redemption_payment".to_string(), v.to_string() );
        }
        if let Some(v) = self.nominal_interest_rate.clone() {
            attributes.insert("nominal_interest_rate".to_string(), v.to_string() );
        }
        if let Some(v) = self.nominal_interest_rate2.clone() {
            attributes.insert("nominal_interest_rate2".to_string(), v.to_string() );
        }
        if let Some(v) = self.notional_principal.clone() {
            attributes.insert("notional_principal".to_string(), v.to_string() );
        }
        if let Some(v) = self.notional_principal2.clone() {
            attributes.insert("notional_principal2".to_string(), v.to_string() );
        }
        if let Some(v) = self.notional_scaling_multiplier.clone() {
            attributes.insert("notional_scaling_multiplier".to_string(), v.to_string() );
        }


        attributes
    }

    pub fn are_computed_results_correct(&self, computed_res: HashMap<String, String>) -> bool {

        let mut resx = true;
        
        for (cle, valeur) in &self.get_expected_results() {
            let v = computed_res.get(&cle.to_string());

            if let (Some(v), valeur) = (v, valeur) {
                if v != valeur {
                    resx = false
                }
            }
        }
        
        resx
    }

}

pub fn load_results(
    file_path: &str,
    test_case_id: &str,
) -> Result<Vec<ResultSet>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json: JsonValue = serde_json::from_reader(reader)?;

    let test_case = json.get(test_case_id)
        .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

    let results = test_case.get("results")
        .ok_or_else(|| format!("'results' section not found in {}", test_case_id))?;

    if let JsonValue::Array(results_array) = results {
        let mut result_sets = Vec::new();

        for result_item in results_array {
            if let JsonValue::Object(item_obj) = result_item {
                let mut result_set = ResultSet {
                    values: HashMap::new(),
                    event_date: None,
                    exercise_date: None,
                    event_type: None,
                    currency: None,
                    payoff: None,
                    accrued_interest: None,
                    accrued_interest2: None,
                    exercise_amount: None,
                    fee_accrued: None,
                    interest_calculation_base_amount: None,
                    interest_scaling_multiplier: None,
                    next_principal_redemption_payment: None,
                    nominal_interest_rate: None,
                    nominal_interest_rate2: None,
                    notional_principal: None,
                    notional_principal2: None,
                    notional_scaling_multiplier: None,
                };

                let mut values = HashMap::new();

                for (key, value) in item_obj {
                    match key.as_str() {
                        "eventDate" => {
                            if let JsonValue::String(s) = value {
                                result_set.event_date = Some(IsoDatetime::from_str(s)?);
                            }
                        },
                        "eventType" => {
                            if let JsonValue::String(s) = value {
                                result_set.event_type = Some(EventType::from_str(s)?);
                            }
                        },
                        "currency" => {
                            if let JsonValue::String(s) = value {
                                result_set.currency = Some(Currency::new(s.clone())?);
                            }
                        },
                        "payoff" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.payoff = Some(f);
                                }
                            }
                        },
                        "accruedInterest" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.accrued_interest = AccruedInterest::new(f).ok();
                                }
                            }
                        },
                        "accruedInterest2" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.accrued_interest2 = AccruedInterest2::new(f).ok();
                                }
                            }
                        },
                        "exerciseAmount" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.exercise_amount = ExerciseAmount::new(f).ok();
                                }
                            }
                        },
                        "feeAccrued" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.fee_accrued = FeeAccrued::new(f).ok();
                                }
                            }
                        },
                        "interestCalculationBaseAmount" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.interest_calculation_base_amount = InterestCalculationBaseAmount::new(f).ok();
                                }
                            }
                        },
                        "interestScalingMultiplier" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.interest_scaling_multiplier = InterestScalingMultiplier::new(f).ok();
                                }
                            }
                        },
                        "nextPrincipalRedemptionPayment" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.next_principal_redemption_payment = NextPrincipalRedemptionPayment::new(f).ok();
                                }
                            }
                        },
                        "nominalInterestRate" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.nominal_interest_rate = NominalInterestRate::new(f).ok();
                                }
                            }
                        },
                        "nominalInterestRate2" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.nominal_interest_rate2 = NominalInterestRate2::new(f).ok();
                                }
                            }
                        },
                        "notionalPrincipal" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.notional_principal = NotionalPrincipal::new(f).ok();
                                }
                            }
                        },
                        "notionalPrincipal2" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.notional_principal2 = NotionalPrincipal2::new(f).ok();
                                }
                            }
                        },
                        "notionalScalingMultiplier" => {
                            if let JsonValue::Number(n) = value {
                                if let Some(f) = n.as_f64() {
                                    result_set.notional_scaling_multiplier = NotionalScalingMultiplier::new(f).ok();
                                }
                            }
                        },
                        "exerciseDate" => {
                            if let JsonValue::String(s) = value {
                                result_set.exercise_date = ExerciseDate::new(IsoDatetime::from_str(s)?).ok();
                            }
                        },
                        _ => {
                            // Stocker les champs supplémentaires dans la HashMap values
                            values.insert(key.clone(), value.to_string());
                        }
                    }
                }

                // Assigner la HashMap des valeurs supplémentaires
                result_set.values = values;
                result_sets.push(result_set);
            }
        }

        Ok(result_sets)
    } else {
        Err("Results should be an array".into())
    }
}

use std::fmt;
use crate::traits::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

impl fmt::Display for ResultSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ResultSet {{")?;

        // Afficher les champs principaux
        if let Some(ref event_date) = self.event_date {
            writeln!(f, "  event_date: {}", event_date)?;
        }
        if let Some(ref event_type) = self.event_type {
            writeln!(f, "  event_type: {}", event_type)?;
        }
        if let Some(ref currency) = self.currency {
            writeln!(f, "  currency: {}", currency)?;
        }
        if let Some(payoff) = self.payoff {
            writeln!(f, "  payoff: {}", payoff)?;
        }

        // Afficher les champs optionnels structurés
        macro_rules! display_optional_field {
            ($field:expr, $name:expr) => {
                if let Some(ref value) = $field {
                    writeln!(f, "  {}: {}", $name, value)?;
                }
            };
        }

        display_optional_field!(self.accrued_interest, "accrued_interest");
        display_optional_field!(self.accrued_interest2, "accrued_interest2");
        display_optional_field!(self.exercise_amount, "exercise_amount");
        display_optional_field!(self.exercise_date, "exercise_date");
        display_optional_field!(self.fee_accrued, "fee_accrued");
        display_optional_field!(self.interest_calculation_base_amount, "interest_calculation_base_amount");
        display_optional_field!(self.interest_scaling_multiplier, "interest_scaling_multiplier");
        display_optional_field!(self.next_principal_redemption_payment, "next_principal_redemption_payment");
        display_optional_field!(self.nominal_interest_rate, "nominal_interest_rate");
        display_optional_field!(self.nominal_interest_rate2, "nominal_interest_rate2");
        display_optional_field!(self.notional_principal, "notional_principal");
        display_optional_field!(self.notional_principal2, "notional_principal2");
        display_optional_field!(self.notional_scaling_multiplier, "notional_scaling_multiplier");

        // Afficher les valeurs supplémentaires
        if !self.values.is_empty() {
            writeln!(f, "  additional_values:")?;
            for (key, value) in &self.values {
                writeln!(f, "    {}: {}", key, value)?;
            }
        }

        write!(f, "}}")
    }
}