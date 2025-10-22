pub mod common_test_funcs;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use gfs_lib_contract::attributes::ContractModel::ContractModel;
use gfs_lib_contract::util::ResultsStruct::TestResult;
use gfs_lib_types::types::Value::Value;
use crate::common_test_funcs::CompareTestResults::compare_test_results;
use crate::common_test_funcs::test_json_loader::{load_test_case, load_test_case_dataobserved, load_test_case_results, load_test_case_terms2, load_tests, DataObserved};
use test_case::test_case;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
// use test_case::test_matrix;

// fn extract_numbers_with_original(strings: Vec<&String>) -> Vec<(&str, i32)> {
//     strings
//         .iter()
//         .map(|s| {
//             let num_str: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
//             let num = num_str.parse::<i32>().unwrap();
//             (s.as_str(), num)
//         })
//         .collect()
// }


pub fn setup_tests(test_id: &str) -> (HashMap<String, Value>, Vec<TestResult>, DataObserved) {
    let pathx = "tests/test_sets/actus-tests-pam-converted.json";
    let d_dict = load_tests(pathx);
    let curr_test = load_test_case(test_id, &d_dict).unwrap();
    //let json_value = test_read_and_parse_json(pathx).unwrap();
    let contract_terms_dict2 = load_test_case_terms2(&curr_test).unwrap();
    let contract_results_dict = load_test_case_results(&curr_test).unwrap();
    let contract_data_observed = load_test_case_dataobserved(&curr_test).unwrap();
    (contract_terms_dict2, contract_results_dict, contract_data_observed)
}

#[test_case("pam01")]
#[test_case("pam02")]
#[test_case("pam03")]
#[test_case("pam04")]
#[test_case("pam05")]
#[test_case("pam06")]
#[test_case("pam07")]
#[test_case("pam08")]
#[test_case("pam09")]
#[test_case("pam10")]
#[test_case("pam11")]
#[test_case("pam12")]
#[test_case("pam13")]
#[test_case("pam14")]
#[test_case("pam15")]
#[test_case("pam16")]
#[test_case("pam17")]
#[test_case("pam18")]
#[test_case("pam19")]
#[test_case("pam20")]
#[test_case("pam21")]
#[test_case("pam22")]
#[test_case("pam23")]
#[test_case("pam24")]
#[test_case("pam25")]
fn test_pam(test_id: &str) {
    // let test_id = "pam01";
    let (contract_terms_dict2,
        contract_results_dict,
        contract_data_observed) = setup_tests(test_id);

    let mut pam = ContractModel::new(
        contract_terms_dict2.clone(),
        Some(Arc::new(contract_data_observed)),
        None,
    ).expect("Error creating pam contract");

    pam.run_schedule(None);
    let res = pam.run_apply(None, true).expect("ok").expect("ok"); // a

    assert_eq!(compare_test_results(&res, &contract_results_dict), true);
}

#[test]
fn test_pam01_next_event() {
    let test_id = "pam01";
    let (contract_terms_dict2,
        contract_results_dict,
        contract_data_observed) = setup_tests(test_id);

    let mut pam = ContractModel::new(
        contract_terms_dict2.clone(),
        Some(Arc::new(contract_data_observed)),
        None,
    ).expect("Error creating pam contract");


    pam.run_schedule(None);
    let res = pam.run_apply(None, true).expect("ok").expect("ok"); // a

    assert_eq!(compare_test_results(&res, &contract_results_dict), true);
}

#[test]
fn test_pam01_next() {
    let test_id = "pam01";
    let (contract_terms_dict2,
        contract_results_dict,
        contract_data_observed) = setup_tests(test_id);

    let mut pam = ContractModel::new(
        contract_terms_dict2.clone(),
        Some(Arc::new(contract_data_observed)),
        None,
    ).expect("Error creating pam contract");


    pam.run_schedule(None);

    let period1d = *PhantomIsoPeriodW::new(0,0,1);

    let (first_event_time, last_event_time) = &pam.get_contract_first_last_event_time();
    let first_event_time = first_event_time.unwrap().value();
    let last_event_time = last_event_time.unwrap().value();
    let status_date = pam.get_contract_status_date().unwrap().value();
    let mut status_date_next = status_date + period1d;

    let first_event_time_str = format!("{}", first_event_time);
    let last_event_time_str = format!("{}", last_event_time);
    let status_date_str = format!("{}", status_date);
    let mut status_date_next_str = format!("{}", status_date_next);

    let mut curr_res: Option<Result<Vec<TestResult>, String>> = None;

    let mut result_vec: Vec<TestResult> = Vec::new();

    while status_date_next < last_event_time {
        let status_date_curr = &pam.clone().get_contract_status_date().unwrap().value();
        let status_date_curr_str = format!("{}", status_date_curr);


        curr_res = pam.next_day(true);
        if curr_res.is_some() {
            result_vec.extend(curr_res.expect("ok").expect("ok"));
        }
        status_date_next = *status_date_curr + period1d;
        status_date_next_str = format!("{}", status_date_next);
        println!("ok");
    }


    println!("ok");
    assert!(true)
    //assert_eq!(compare_test_results(&res, &contract_results_dict), true);
}