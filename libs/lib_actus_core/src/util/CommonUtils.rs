use std::collections::HashMap;
use std::str::FromStr;
use crate::attributes::ContractTerms::ContractTerms;
use crate::util::Value::Value;
use crate::exceptions::ParseError::ParseError;
use crate::external::RiskFactorModel::{RiskFactorModel};
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::types::IsoDatetime::IsoDatetime;


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


    pub fn settlementCurrencyFxRate(riskFactorModel: &RiskFactorModel, model: &ContractTerms, time: &IsoDatetime, state: &StateSpace) -> f64{
        let settlement_currency = model.settlement_currency.clone();
        let currency = model.currency.clone();
        
        if settlement_currency.is_none()  || currency == Some(settlement_currency.clone().unwrap().to_currency()) {
            1.0
        }
        else {
            let strings = vec![currency.unwrap(), settlement_currency.clone().unwrap().to_currency()]; // refaire plus proprement pour pas melanger Currency et setllment currency

            let str_slices: Vec<String> = strings.iter().map(|s| s.value()).collect();
            let joined = str_slices.join(" ");

            riskFactorModel.state_at(joined, time, state, model,true).expect("expect curr value")
        }
        
    }
}