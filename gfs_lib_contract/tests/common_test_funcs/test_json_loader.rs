use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};
use gfs_lib_contract::traits::TraitExternalData::TraitExternalData;
use gfs_lib_contract::util::ResultsStruct::TestResult;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_types::types::Value::{ContractStructure, Value};

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

//Fonction publique pour charger les termes
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

pub fn load_test_case_terms2(
    test_case: &TestCase,
) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {

    let terms = test_case.clone().terms.0;

    let mut result_map = HashMap::new();
    for (key, value) in terms {
        let val_to_ins = match value {
            TermsValue::String(v) => {
                Value::Vstring(v.clone())
            },
            TermsValue::ContractStructure(v) => {
                Value::VvecCs(v)
            },
        };

        result_map.insert(key.clone(), val_to_ins);
    }
    Ok(result_map)

}


pub fn load_test_case_contract_structure(
    test_case: &TestCase,
) -> Option<Vec<ContractStructure>> {

    let a = test_case.clone().terms.clone().0;
    let b = a.get("contractStructure");

    if let Some(c) = b.cloned() {
        let cs: Result<Vec<ContractStructure>, String> = match c {
            TermsValue::ContractStructure(v) => Ok(v),
            _ => Err("Invalid 'contractStructure' format".into())
        };
        match cs {
            Ok(v) => {
                Some(v)
            },
            Err(e) => {
                println!("Erreur : {}", e);
                None
            }
        }
    } else {
        None
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
pub struct DataObserved(HashMap<String, DataStruct>);

impl TraitExternalData for DataObserved {
    /// Returns the set of unique risk factor IDs
    fn keys(&self) -> Option<HashSet<String>> {
        let a: HashSet<String> = self.0.keys().cloned().collect();
        Some(a)
    }

    /// Returns the state of a particular risk factor at a future time
    fn state_at(
        &self,
        id: String,
        time: &PhantomIsoDatetimeW
    ) -> Option<f64> {
        let a = self.0.get(&id)?;
        let mut b: Option<f64> = None;
        for d in a.data.iter() {
            let fdfgds = time.to_string();
            if d.timestamp == time.to_string() {
                b = d.value.clone().parse::<f64>().ok()
            }
        };
        b
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum TermsValue {
    String(String),
    ContractStructure(Vec<ContractStructure>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Terms(HashMap<String, TermsValue>);

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestCase {
    pub identifier: String,
    pub terms: Terms,
    pub to: String,
    pub dataObserved: DataObserved, // Pas une HashMap imbriquée, mais une HashMap<String, DataStruct>
    pub eventsObserved: Vec<HashMap<String, String>>,
    pub results: Vec<TestResult>,
}

pub fn load_test_case_results(
    test_case: &TestCase,
) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {

    Ok(test_case.results.clone())
}

pub fn load_test_case_dataobserved(
    test_case: &TestCase,
) -> Result<DataObserved, Box<dyn std::error::Error>> {

    Ok(test_case.dataObserved.clone())
}


// pub fn load_test_case_terms(
//     test_case: &TestCase,
// ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
//     Ok(test_case.terms.clone())
// }

// Version alternative si vous voulez aussi les termes
pub fn load_test_case(
    test_case_id: &str,
    tests: &HashMap<String, TestCase>,
) -> Result<TestCase, Box<dyn std::error::Error>> {

    let test_case = tests.get(test_case_id)
        .ok_or_else(|| format!("Test case {} not found", test_case_id))?;

    let r = test_case.clone().clone();
    // Clone le test case complet
    Ok(r)
}


pub fn load_tests(file_path: &str) -> HashMap<String, TestCase> {
    let file = File::open(file_path);
    let reader = BufReader::new(file.expect("Unable to open file"));

    // Parse le JSON en HashMap<String, TestCase>
    let json: Result<HashMap<String, TestCase>, _> = serde_json::from_reader(reader);
    json.expect("Unable to parse JSON")
}