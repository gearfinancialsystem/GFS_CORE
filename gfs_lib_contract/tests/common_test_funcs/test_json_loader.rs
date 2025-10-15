use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};
use gfs_lib_contract::util::ResultsStruct::TestResult;
use gfs_lib_types::types::Value::Value;

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

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct TestResult {
//     pub eventDate: String,
//     pub eventType: String,
//     pub payoff: String,
//     pub currency: String,
//     pub notionalPrincipal: String,
//     pub nominalInterestRate: String,
//     pub accruedInterest: String,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPoint {
    pub timestamp: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataStruct {
    pub identifier: String,
    pub data: Vec<DataPoint>, // C'est un Vec<DataPoint>, pas un DataPoint
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestCase {
    pub identifier: String,
    pub terms: HashMap<String, String>,
    pub to: String,
    pub dataObserved: HashMap<String, DataStruct>, // Pas une HashMap imbriquée, mais une HashMap<String, DataStruct>
    pub eventsObserved: Vec<HashMap<String, String>>,
    pub results: Vec<TestResult>,
}
pub fn load_test_case_results(
    file_path: &str,
    test_case_id: &str,
) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
    // Ouvre le fichier
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Parse le JSON en HashMap<String, TestCase>
    let json: HashMap<String, TestCase> = serde_json::from_reader(reader)?;
    // println!("{:?}", json);
    // Récupère le test case spécifique
    let test_case = json.get(test_case_id)
        .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

    // Retourne les résultats
    Ok(test_case.results.clone())
}

// Version alternative si vous voulez aussi les termes
pub fn load_test_case(
    file_path: &str,
    test_case_id: &str,
) -> Result<TestCase, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let json: HashMap<String, TestCase> = serde_json::from_reader(reader)?;

    let test_case = json.get(test_case_id)
        .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

    let r = test_case.clone().clone();
    // Clone le test case complet
    Ok(r)
}