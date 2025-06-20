use std::collections::HashMap;
use lib_actus_core::types::isoDatetime::IsoDatetime;
use lib_actus_core::attributes::ContractModel::ContractModel;
use lib_actus_core::contracts::PrincipalAtMaturity::PrincipalAtMaturity;
use lib_actus_core::externals::RiskFactorModel::RiskFactorModel;
use lib_actus_core::terms::grp_contract_identification::contract_types::Pam;

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
    //dico.insert("contractPerformance".to_string(),"xzxz".to_string());

    let mut dic_parsed = Pam::PAM::init();
    dic_parsed.parse_from_dict(&dico);

    let contract_model = Box::new(ContractModel::new(&dico));

    let to_date = IsoDatetime::parse_from_str("2014-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    
    let risk_factor_model = RiskFactorModel;
        
    if let Ok(cm) = contract_model.as_ref().as_ref() {
        let mut events = PrincipalAtMaturity::schedule(&to_date, cm);
        if let Ok(events_res) = &mut events {
            let events2 = PrincipalAtMaturity::apply(events_res, cm, &risk_factor_model);
            
            for ce in events2.iter() {
                println!("EventTime: {:?} - EventType: {:?} - Payoff: {:?} - State.AccruedInterest: {:?}\n", ce.eventTime.unwrap(), ce.eventType, ce.payoff, ce.state.accruedInterest);
                // println!("{} - {}", ce.eventTime.as_ref().unwrap(), ce.payoff.as_ref().unwrap());
                // {'payoff': '-2800.0',
                //     'accruedInterest': '0.0',
                //     'exerciseDate': 'null',
                //     'nominalInterestRate': '0.1',
                //     'eventType': 'IED',
                //     'interestScalingMultiplier': '1.0',
                //     'interestCalculationBaseAmount': '0.0',
                //     'nominalInterestRate2': '0.0',
                //     'notionalScalingMultiplier': '1.0',
                //     'notionalPrincipal2': '0.0',
                //     'notionalPrincipal': '3000.0',
                //     'lastInterestPeriod': '0.0',
                //     'currency': 'USD',
                //     'accruedInterest2': '0.0',
                //     'feeAccrued': '0.0',
                //     'nextPrincipalRedemptionPayment': '0.0',
                //     'nonPerformingDate': 'null',
                //     'exerciseAmount': '0.0',
                //     'eventDate': '2013-01-01T00:00'}
                //println!("ok");
            }

        }
    }

    println!("ok");


}



