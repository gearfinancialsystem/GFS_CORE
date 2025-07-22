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


