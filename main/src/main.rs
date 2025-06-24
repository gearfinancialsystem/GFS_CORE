use std::collections::HashMap;
use std::hash::Hash;
use lib_actus_core::types::isoDatetime::IsoDatetime;
use lib_actus_core::attributes::ContractModel::ContractModel;
use lib_actus_core::contracts::PrincipalAtMaturity::PrincipalAtMaturity;
use lib_actus_core::contracts::Swap::Swap;
use lib_actus_core::externals::RiskFactorModel::RiskFactorModel;
use lib_actus_core::util::CommonUtils::Value;

fn main() {
    let mut dico= HashMap::new(); // HashMap<String, String>

    // PAM02
    // dico.insert("contractType".to_string(),Value::String("PAM".to_string()));
    // dico.insert("contractID".to_string(),Value::String("pam02".to_string()));
    // dico.insert("statusDate".to_string(),Value::String("2012-12-30T00:00:00".to_string()));
    // dico.insert("contractDealDate".to_string(),Value::String("2012-12-28T00:00:00".to_string()));
    // dico.insert("currency".to_string(),Value::String("USD".to_string()));
    // dico.insert("notionalPrincipal".to_string(),Value::String("3000".to_string()));
    // dico.insert("initialExchangeDate".to_string(),Value::String("2013-01-01T00:00:00".to_string()));
    // dico.insert("maturityDate".to_string(),Value::String("2014-01-01T00:00:00".to_string()));
    // dico.insert("nominalInterestRate".to_string(),Value::String("0.1".to_string()));
    // dico.insert("cycleAnchorDateOfInterestPayment".to_string(),Value::String("2013-01-01T00:00:00".to_string()));
    // dico.insert("cycleOfInterestPayment".to_string(),Value::String("P2ML0".to_string()));
    // dico.insert("dayCountConvention".to_string(),Value::String("A360".to_string()));
    // dico.insert("endOfMonthConvention".to_string(),Value::String("SD".to_string()));
    // dico.insert("premiumDiscountAtIED".to_string(),Value::String("-200".to_string()));
    // dico.insert("rateMultiplier".to_string(),Value::String("1.0".to_string()));
    // dico.insert("contractRole".to_string(),Value::String("RPA".to_string()));
    //dico.insert("contractPerformance".to_string(),"xzxz".to_string());

    // SWAPS01
    dico.insert("contractType".to_string(), Value::String("SWAPS".to_string()));
    dico.insert("contractID".to_string(), Value::String("swaps01".to_string()));
    dico.insert("contractRole".to_string(), Value::String("RFL".to_string()));
    dico.insert("currency".to_string(), Value::String("USD".to_string()));
    dico.insert("contractDealDate".to_string(), Value::String("2012-12-28T00:00:00".to_string()));
    dico.insert("statusDate".to_string(), Value::String("2012-12-30T00:00:00".to_string()));
    dico.insert("deliverySettlement".to_string(), Value::String("D".to_string()));
    
    let mut cs1 = HashMap::new();
    let mut obj1 = HashMap::new();
    
    obj1.insert("contractType".to_string(), Value::String("PAM".to_string()));
    obj1.insert("contractID".to_string(), Value::String("swaps01-leg1".to_string()));
    obj1.insert("contractDealDate".to_string(), Value::String("2012-12-28T00:00:00".to_string()));
    obj1.insert("initialExchangeDate".to_string(), Value::String("2013-01-01T00:00:00".to_string()));
    obj1.insert("currency".to_string(), Value::String("USD".to_string()));
    obj1.insert("statusDate".to_string(), Value::String("2012-12-30T00:00:00".to_string()));
    obj1.insert("notionalPrincipal".to_string(), Value::String("1000".to_string()));
    obj1.insert("dayCountConvention".to_string(), Value::String("A365".to_string()));
    obj1.insert("nominalInterestRate".to_string(), Value::String("0.1".to_string()));
    obj1.insert("maturityDate".to_string(), Value::String("2014-01-01T00:00:00".to_string()));
    obj1.insert("cycleAnchorDateOfInterestPayment".to_string(), Value::String("2013-01-01T00:00:00".to_string()));
    obj1.insert("cycleOfInterestPayment".to_string(), Value::String("P1ML1".to_string()));
    obj1.insert("premiumDiscountAtIED".to_string(), Value::String("0.0".to_string()));
    
    cs1.insert("object".to_string(), Value::HashMap(obj1));
    cs1.insert("referenceType".to_string(), Value::String("CNT".to_string()));
    cs1.insert("referenceRole".to_string(), Value::String("FIL".to_string()));
    
    let mut cs2 = HashMap::new();
    let mut obj2 = HashMap::new();
    
    obj2.insert("contractType".to_string(), Value::String("PAM".to_string()));
    obj2.insert("contractID".to_string(), Value::String("swaps01-leg2".to_string()));
    obj2.insert("contractDealDate".to_string(), Value::String("2012-12-28T00:00:00".to_string()));
    obj2.insert("initialExchangeDate".to_string(), Value::String("2013-01-01T00:00:00".to_string()));
    obj2.insert("currency".to_string(), Value::String("USD".to_string()));
    obj2.insert("statusDate".to_string(), Value::String("2012-12-30T00:00:00".to_string()));
    obj2.insert("notionalPrincipal".to_string(), Value::String("1200".to_string()));
    obj2.insert("dayCountConvention".to_string(), Value::String("A365".to_string()));
    obj2.insert("nominalInterestRate".to_string(), Value::String("0.1".to_string()));
    obj2.insert("maturityDate".to_string(), Value::String("2014-01-01T00:00:00".to_string()));
    obj2.insert("cycleAnchorDateOfInterestPayment".to_string(), Value::String("2013-01-01T00:00:00".to_string()));
    obj2.insert("cycleOfInterestPayment".to_string(), Value::String("P3ML1".to_string()));
    obj2.insert("premiumDiscountAtIED".to_string(), Value::String("0.0".to_string()));
    
    cs2.insert("object".to_string(), Value::HashMap(obj2));
    cs2.insert("referenceType".to_string(), Value::String("CNT".to_string()));
    cs2.insert("referenceRole".to_string(), Value::String("SEL".to_string()));
    
    // dico.insert("contractStructure".to_string(), Value::Vec(vec![Value::HashMap(cs1), Value::HashMap(cs2)]));




    let mut contract_model = Box::new(ContractModel::new(&dico));

    //let contract_model = Box::new(ContractModel::new(&dico));

    let to_date = IsoDatetime::parse_from_str("2014-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
    let risk_factor_model = RiskFactorModel;
        
    if let Ok(cm) = contract_model.as_ref() {
        let mut events = Swap::schedule(&to_date, cm); //PrincipalAtMaturity::schedule(&to_date, cm);
        if let Ok(events_res) = events {
            let events2 = PrincipalAtMaturity::apply(events_res, cm, &risk_factor_model);
            
            for ce in events2.iter() {
                println!("EventTime: {:?} - EventType: {:?} - Payoff: {:?} - State.AccruedInterest: {:?}\n", ce.eventTime.unwrap(), ce.eventType, ce.payoff, ce.state.accruedInterest);
            }

        }
    }

    println!("ok");


}



