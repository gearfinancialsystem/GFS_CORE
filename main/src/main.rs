use std::collections::HashMap;
use lib_actus_core::types::IsoDatetime::IsoDatetime;
use lib_actus_core::attributes::ContractModel::ContractModel;
// use lib_actus_core::terms::grp_contract_identification::contract_types::Swaps::SWAPS;
use lib_actus_core::terms::grp_contract_identification::contract_types::Pam::PAM;
use lib_actus_core::externals::RiskFactorModel::RiskFactorModel;
use lib_actus_core::util::Value::Value;


fn main() {
    let mut dico= HashMap::new(); // HashMap<String, String>

    // PAM02
    dico.insert("contractType".to_string(),Value::Vstring("PAM".to_string()));
    dico.insert("contractID".to_string(),Value::Vstring("pam02".to_string()));
    dico.insert("statusDate".to_string(),Value::Vstring("2012-12-30T00:00:00".to_string()));
    dico.insert("contractDealDate".to_string(),Value::Vstring("2012-12-28T00:00:00".to_string()));
    dico.insert("currency".to_string(),Value::Vstring("USD".to_string()));
    dico.insert("notionalPrincipal".to_string(),Value::Vstring("3000".to_string()));
    dico.insert("initialExchangeDate".to_string(),Value::Vstring("2013-01-01T00:00:00".to_string()));
    dico.insert("maturityDate".to_string(),Value::Vstring("2014-01-01T00:00:00".to_string()));
    dico.insert("nominalInterestRate".to_string(),Value::Vstring("0.1".to_string()));
    dico.insert("cycleAnchorDateOfInterestPayment".to_string(),Value::Vstring("2013-01-01T00:00:00".to_string()));
    dico.insert("cycleOfInterestPayment".to_string(),Value::Vstring("P2ML0".to_string()));
    dico.insert("dayCountConvention".to_string(),Value::Vstring("A360".to_string()));
    dico.insert("endOfMonthConvention".to_string(),Value::Vstring("SD".to_string()));
    dico.insert("premiumDiscountAtIED".to_string(),Value::Vstring("-200".to_string()));
    dico.insert("rateMultiplier".to_string(),Value::Vstring("1.0".to_string()));
    dico.insert("contractRole".to_string(),Value::Vstring("RPA".to_string()));
    //dico.insert("contractPerformance".to_string(),"xzxz".to_string());

    // // SWAPS01
    // dico.insert("contractType".to_string(), Value::Vstring("SWAPS".to_string()));
    // dico.insert("contractID".to_string(), Value::Vstring("swaps01".to_string()));
    // dico.insert("contractRole".to_string(), Value::Vstring("RFL".to_string()));
    // dico.insert("currency".to_string(), Value::Vstring("USD".to_string()));
    // dico.insert("contractDealDate".to_string(), Value::Vstring("2012-12-28T00:00:00".to_string()));
    // dico.insert("statusDate".to_string(), Value::Vstring("2012-12-30T00:00:00".to_string()));
    // dico.insert("deliverySettlement".to_string(), Value::Vstring("D".to_string()));
    
    // let mut cs1 = HashMap::new();
    // let mut obj1 = HashMap::new();
    
    // obj1.insert("contractType".to_string(), Value::Vstring("PAM".to_string()));
    // obj1.insert("contractID".to_string(), Value::Vstring("swaps01-leg1".to_string()));
    // obj1.insert("contractDealDate".to_string(), Value::Vstring("2012-12-28T00:00:00".to_string()));
    // obj1.insert("initialExchangeDate".to_string(), Value::Vstring("2013-01-01T00:00:00".to_string()));
    // obj1.insert("currency".to_string(), Value::Vstring("USD".to_string()));
    // obj1.insert("statusDate".to_string(), Value::Vstring("2012-12-30T00:00:00".to_string()));
    // obj1.insert("notionalPrincipal".to_string(), Value::Vstring("1000".to_string()));
    // obj1.insert("dayCountConvention".to_string(), Value::Vstring("A365".to_string()));
    // obj1.insert("nominalInterestRate".to_string(), Value::Vstring("0.1".to_string()));
    // obj1.insert("maturityDate".to_string(), Value::Vstring("2014-01-01T00:00:00".to_string()));
    // obj1.insert("cycleAnchorDateOfInterestPayment".to_string(), Value::Vstring("2013-01-01T00:00:00".to_string()));
    // obj1.insert("cycleOfInterestPayment".to_string(), Value::Vstring("P1ML1".to_string()));
    // obj1.insert("premiumDiscountAtIED".to_string(), Value::Vstring("0.0".to_string()));
    
    // cs1.insert("object".to_string(), Value::VhashMap(obj1));
    // cs1.insert("referenceType".to_string(), Value::Vstring("CNT".to_string()));
    // cs1.insert("referenceRole".to_string(), Value::Vstring("FIL".to_string()));
    
    // let mut cs2 = HashMap::new();
    // let mut obj2 = HashMap::new();
    
    // obj2.insert("contractType".to_string(), Value::Vstring("PAM".to_string()));
    // obj2.insert("contractID".to_string(), Value::Vstring("swaps01-leg2".to_string()));
    // obj2.insert("contractDealDate".to_string(), Value::Vstring("2012-12-28T00:00:00".to_string()));
    // obj2.insert("initialExchangeDate".to_string(), Value::Vstring("2013-01-01T00:00:00".to_string()));
    // obj2.insert("currency".to_string(), Value::Vstring("USD".to_string()));
    // obj2.insert("statusDate".to_string(), Value::Vstring("2012-12-30T00:00:00".to_string()));
    // obj2.insert("notionalPrincipal".to_string(), Value::Vstring("1200".to_string()));
    // obj2.insert("dayCountConvention".to_string(), Value::Vstring("A365".to_string()));
    // obj2.insert("nominalInterestRate".to_string(), Value::Vstring("0.1".to_string()));
    // obj2.insert("maturityDate".to_string(), Value::Vstring("2014-01-01T00:00:00".to_string()));
    // obj2.insert("cycleAnchorDateOfInterestPayment".to_string(), Value::Vstring("2013-01-01T00:00:00".to_string()));
    // obj2.insert("cycleOfInterestPayment".to_string(), Value::Vstring("P3ML1".to_string()));
    // obj2.insert("premiumDiscountAtIED".to_string(), Value::Vstring("0.0".to_string()));
    
    // cs2.insert("object".to_string(), Value::VhashMap(obj2));
    // cs2.insert("referenceType".to_string(), Value::Vstring("CNT".to_string()));
    // cs2.insert("referenceRole".to_string(), Value::Vstring("SEL".to_string()));


    // let mut v: Vec<Value> = Vec::new();
    // v.push(Value::VhashMap(cs1.clone()));
    // v.push(Value::VhashMap(cs2.clone()));

    // dico.insert("contractStructure".to_string(), Value::VvecVal(v));

    // CASH 01
    // dico.insert("contractType".to_string(),Value::Vstring("CSH".to_string()));
    // dico.insert("contractID".to_string(),Value::Vstring("csh01".to_string()));
    // dico.insert("statusDate".to_string(),Value::Vstring("2015-07-15T00:00:00".to_string()));
    // dico.insert("contractRole".to_string(),Value::Vstring("RPA".to_string()));
    // dico.insert("currency".to_string(),Value::Vstring("CHF".to_string()));
    // dico.insert("notionalPrincipal".to_string(),Value::Vstring("1000".to_string()));


    // test loading avec functions
    // let pathx = "libs/lib_actus_core/tests_sets/actus-tests-swaps.json";
    // let json_value = test_read_and_parse_json(pathx).unwrap();
    // let dico_from_json = json_to_dico(json_value);
    // 
    // let first_val = dico_from_json.get("swaps01").unwrap().extract_hmap().unwrap();
    // let terms = first_val.get("terms").unwrap().extract_hmap().unwrap();
    // 
    // let contracts_structs = terms.get("contractStructure").unwrap().extract_vec().unwrap();
    // let first_contract = contracts_structs.get(0).unwrap();
    let mut contract_model = Box::new(ContractModel::new(&dico));

    //let contract_model = Box::new(ContractModel::new(&dico));

    let to_date = IsoDatetime::parse_from_str("2014-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
    let risk_factor_model = RiskFactorModel;
    
    if let Ok(cm) = contract_model.as_ref() {
        let mut events = PAM::schedule(&to_date, cm); //PrincipalAtMaturity::schedule(&to_date, cm);
        if let Ok(events_res) = events {
            let events2 = PAM::apply(events_res, cm, &risk_factor_model);
            
            for ce in events2.iter() {
                println!("EventTime: {:?} - EventType: {:?} - Payoff: {:?} - State.AccruedInterest: {:?}\n", ce.event_time.unwrap(), ce.event_type, ce.payoff, ce.state.accrued_interest);
            }

        }
    }

    println!("ok");


}



