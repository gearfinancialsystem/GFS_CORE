pub mod common_test_funcs;

use std::str::FromStr;
use gfs_lib_contract::attributes::ContractModel::ContractModel;
use gfs_lib_contract::traits::TraitContractModel::TraitContractModel;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::common_test_funcs::CompareTestResults::compare_test_results;
use crate::common_test_funcs::test_json_loader::{load_test_case_results, load_test_case_terms};

#[test]
fn test_pam_contract_creation() {
    // Créez une instance de PAM avec des paramètres de test

    let pathx = "tests/test_sets/actus-tests-pam-converted.json";
    let test_id = "pam12"; //12

    //let json_value = test_read_and_parse_json(pathx).unwrap();
    let contract_terms_dict = load_test_case_terms(pathx,test_id).unwrap();

    let contract_results_dict = load_test_case_results(pathx,test_id).unwrap();


    let mut pam = ContractModel::new(
        &contract_terms_dict, 
        None,
        None,
        true
    ).expect("Error creating pam contract");
    
    // Initialisez les termes du contrat avec des données de test
    let a = PhantomIsoDatetimeW::from_str("2013-06-14T00:00:00").ok();
    pam.run_schedule();
    let res = pam.run_apply(None, true); // a
    let res_dispo = res.expect("ok").expect("ok");

    //println!("{:?}", res_dispo);
    assert_eq!(compare_test_results(&res_dispo, &contract_results_dict), true);
    println!("ok")
}