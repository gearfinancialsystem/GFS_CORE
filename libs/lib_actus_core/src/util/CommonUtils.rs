use std::collections::HashMap;
use std::str::FromStr;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::exceptions::ParseError::ParseError;
use crate::external::RiskFactorModel::{RiskFactorModel};

use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;



pub struct CommonUtils;

pub const CURRENCIES: [&str; 169] = [
    "AED", "AFN", "ALL", "AMD", "AOA", "ARS", "AUD", "AWG", "AZN",
    "BAM", "BBD", "BDT", "BGN", "BHD", "BIF", "BMD", "BND", "BOB",
    "BOV", "BRL", "BSD", "BTN", "BWP", "BYN", "BZD", "CAD", "CDF",
    "CHE", "CHF", "CHW", "CLF", "CLP", "CNY", "COP", "COU", "CRC",
    "CUC", "CUP", "CVE", "CZK", "DJF", "DKK", "DOP", "DZD", "EGP",
    "ERN", "ETB", "EUR", "FJD", "FKP", "GBP", "GEL", "GHS", "GIP",
    "GMD", "GNF", "GTQ", "GYD", "HKD", "HNL", "HTG", "HUF", "IDR",
    "ILS", "INR", "IQD", "IRR", "ISK", "JMD", "JOD", "JPY", "KES",
    "KGS", "KHR", "KMF", "KPW", "KRW", "KWD", "KYD", "KZT", "LAK",
    "LBP", "LKR", "LRD", "LSL", "LYD", "MAD", "MDL", "MGA", "MKD",
    "MMK", "MNT", "MOP", "MRU", "MUR", "MVR", "MWK", "MXN", "MXV",
    "MYR", "MZN", "NAD", "NGN", "NIO", "NOK", "NPR", "NZD", "OMR",
    "PAB", "PEN", "PGK", "PHP", "PKR", "PLN", "PYG", "QAR", "RON",
    "RSD", "RUB", "RWF", "SAR", "SBD", "SCR", "SDG", "SEK", "SGD",
    "SHP", "SLE", "SOS", "SRD", "SSP", "STN", "SVC", "SYP", "SZL",
    "THB", "TJS", "TMT", "TND", "TOP", "TRY", "TTD", "TWD", "TZS",
    "UAH", "UGX", "USD", "USN", "UYI", "UYU", "UZS", "VED", "VEF",
    "VND", "VUV", "WST", "XAF", "XAU", "XCD", "XCG", "XDR", "XOF",
    "XPF", "XSU", "XUA", "YER", "ZAR", "ZMW", "ZWL",
];

impl CommonUtils {


    pub fn settlementCurrencyFxRate(riskFactorModel: Option<&dyn TraitRiskFactorModel>, 
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
                riskFactorModel.unwrap().state_at(joined, time, state, model,true).expect("expect curr value")
            } else { 
                1.0 // a verifier
            }


            
        }
        
    }
}