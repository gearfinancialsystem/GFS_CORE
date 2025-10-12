pub mod common_test_funcs;

use std::str::FromStr;
use gfs_lib_contract::attributes::ContractModel::ContractModel;
use gfs_lib_contract::traits::TraitContractModel::TraitContractModel;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::common_test_funcs::test_json_loader::load_test_case_terms;

#[test]
fn test_pam_contract_creation() {
    // Créez une instance de PAM avec des paramètres de test

    let pathx = "tests/test_sets/actus-tests-pam.json";
    let test_id = "pam01";

    //let json_value = test_read_and_parse_json(pathx).unwrap();
    let contract_terms_dict = load_test_case_terms(pathx,test_id).unwrap();
    
    let mut pam = ContractModel::new(
        &contract_terms_dict, 
        None,
        None,
        true
    ).expect("Error creating pam contract");
    
    // Initialisez les termes du contrat avec des données de test
    let a = PhantomIsoDatetimeW::from_str("2014-06-14T00:00:00").ok();
    pam.run_apply(a);
    // Vérifiez que l'initialisation s'est bien passée
    println!("ok")
    //assert_eq!(pam.contract_id.to_string(), "init");
}