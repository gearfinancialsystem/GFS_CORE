pub mod common_test_funcs;

use std::sync::Arc;
use gfs_lib_contract::attributes::ContractModel::ContractModel;
use crate::common_test_funcs::CompareTestResults::compare_test_results;
use crate::common_test_funcs::test_json_loader::{load_test_case, load_test_case_dataobserved, load_test_case_results, load_test_case_terms, load_test_case_terms2, load_tests};

fn extract_numbers_with_original(strings: Vec<&String>) -> Vec<(&str, i32)> {
    strings
        .iter()
        .map(|s| {
            let num_str: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = num_str.parse::<i32>().unwrap();
            (s.as_str(), num)
        })
        .collect()
}


#[test]
fn test_ann_contract_creation() {
    // Créez une instance de ANN avec des paramètres de test

    let pathx = "tests/test_sets/actus-tests-ann-converted.json";
    let d_dict = load_tests(pathx);
    let mut test_vec = extract_numbers_with_original(d_dict.keys().collect::<Vec<&String>>());
    test_vec.sort_by(|a, b| a.1.cmp(&b.1));

    for (test_id, _) in test_vec {
        let curr_test = load_test_case(test_id, &d_dict).unwrap();
        //let json_value = test_read_and_parse_json(pathx).unwrap();
        let contract_terms_dict2 = load_test_case_terms2(&curr_test).unwrap();
        let contract_results_dict = load_test_case_results(&curr_test).unwrap();
        let contract_data_observed = load_test_case_dataobserved(&curr_test).unwrap();
        
        // let testaa = contract_data_observed.clone().state_at("USD_SWP".to_string(), &PhantomIsoDatetimeW::from_str("2013-02-01T00:00:00").unwrap() );

        let mut ann = ContractModel::new(
            contract_terms_dict2.clone(),
            Some(Arc::new(contract_data_observed)),
            None,
        ).expect("Error creating ann contract");

        // Initialisez les termes du contrat avec des données de test
        // let a = PhantomIsoDatetimeW::from_str("2013-06-14T00:00:00").ok();
        ann.run_schedule(None);
        let res = ann.run_apply(None, true); // a
        let res_dispo = res.expect("ok").expect("ok");

        //println!("{:?}", res_dispo);
        assert_eq!(compare_test_results(&res_dispo, &contract_results_dict), true);
        // println!("{:?}", test_id);
    }

}