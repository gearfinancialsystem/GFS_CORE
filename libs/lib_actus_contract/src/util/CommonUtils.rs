use std::collections::HashMap;
use std::str::FromStr;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::exceptions::ParseError::ParseError;
use crate::external::RiskFactorModel::{RiskFactorModel};

use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;



pub struct CommonUtils;


impl CommonUtils {


    pub fn settlementCurrencyFxRate(riskFactorModel: &Option<RiskFactorModel>, 
                                    model: &ContractTerms, 
                                    time: &IsoDatetime, 
                                    state: &StatesSpace) -> f64{
        let settlement_currency = model.settlement_currency.clone();
        let currency = model.currency.clone();
        
        if settlement_currency.is_none()  || currency == Some(settlement_currency.clone().unwrap().to_currency()) {
            1.0
        }
        else {
            let strings = vec![currency.unwrap(), settlement_currency.clone().unwrap().to_currency()]; // refaire plus proprement pour pas melanger Currency et setllment currency

            let str_slices: Vec<String> = strings.iter().map(|s| s.value()).collect();
            let joined = str_slices.join(" ");
            
            if riskFactorModel.is_none() {
                riskFactorModel.clone().unwrap().state_at(joined, time, state, model,true).expect("expect curr value")
            } else { 
                1.0 // a verifier
            }


            
        }
        
    }
}