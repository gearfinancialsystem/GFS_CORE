use std::collections::HashMap;
// use lib1::hello_from_lib1;
// use lib2::hello_from_lib2;
use lib_actus_core::time::adjusters::{WeekdayCycleAdjuster::WeekdayCycleAdjuster, PeriodCycleAdjuster::PeriodCycleAdjuster};
use lib_actus_core::time::CycleAdjuster::CycleAdjuster;
use lib_actus_core::traits::TraitCycleAdjuster::TraitCycleAdjuster;
use lib_actus_core::types::isoDatetime::IsoDatetime;

fn main() {
    let mut dico: HashMap<String, String> = HashMap::new();

    dico.insert("contractType".to_string(),"PAM".to_string());
    dico.insert("contractID".to_string(),"pam02".to_string());
    dico.insert("statusDate".to_string(),"2012-12-30T00:00:00".to_string());
    dico.insert("contractDealDate".to_string(),"2012-12-28T00:00:00".to_string());
    dico.insert("currency".to_string(),"USD".to_string());
    dico.insert("notionalPrincipal".to_string(),"3000".to_string());
    dico.insert("initialExchangeDate".to_string(),"2013-01-01T00:00:00".to_string());
    dico.insert("maturityDate".to_string(),"2014-01-01T00:00:00".to_string());
    dico.insert("nominalInterestRate".to_string(),"0.1".to_string());
    dico.insert("cycleAnchorDateOfInterestPayment".to_string(),"2013-01-01T00:00:00".to_string());
    dico.insert("cycleOfInterestPayment".to_string(),"P2ML0".to_string());
    dico.insert("dayCountConvention".to_string(),"A360".to_string());
    dico.insert("endOfMonthConvention".to_string(),"SD".to_string());
    dico.insert("premiumDiscountAtIED".to_string(),"-200".to_string());
    dico.insert("rateMultiplier".to_string(),"1.0".to_string());
    dico.insert("contractRole".to_string(),"RPA".to_string());
    
    let dic_parsed = ContractModel::parse(&dico);
}



