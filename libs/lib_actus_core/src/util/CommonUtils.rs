use std::collections::HashMap;
use std::str::FromStr;
use crate::attributes::ContractModel::ContractModel;
use crate::util::Value::Value;
use crate::exceptions::ParseError::ParseError;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
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

    pub fn provide_bool(string_map: &HashMap<String, Value>, key: &str) -> Option<bool> {
        // string_map.get(key).and_then(|s| s.parse::<f64>().ok())
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match <bool>::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }

    pub fn provide_f64(string_map: &HashMap<String, Value>, key: &str) -> Option<f64> {
        // string_map.get(key).and_then(|s| s.parse::<f64>().ok())
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match <f64>::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }

    pub fn provide_f64default(string_map: &HashMap<String, Value>, key: &str, default: f64) -> Option<f64> {
        match string_map.get(key) {
            None => Some(default), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match <f64>::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }

    pub fn provide_string(string_map: &HashMap<String, Value>, key: &str) -> Option<String> {
        println!("{:?}", key);
        //string_map.get(key).unwrap().as_string()


        string_map
            .get(key)
            .and_then(|s| {
                Some(s.as_string().unwrap().to_string())
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }

    pub fn provide_string_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<String>> {
        println!("{:?}", key);
        //string_map.get(key).unwrap().as_string()


        string_map
            .get(key)
            .and_then(|s| {
                Some(vec![s.to_string()])
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }

    pub fn provide_f64_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<f64>> {
        println!("{:?}", key);
        //string_map.get(key).unwrap().as_string()


        string_map
            .get(key)
            .and_then(|s| {
                Some(vec![s.to_f64()])
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }

    pub fn provide<T>(string_map: &HashMap<String, Value>, key: &str) -> Option<T>
    where
        T: FromStr<Err = ParseError> + Default,
    {
        match string_map.get(key) {
            None => Some(T::default()), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match T::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(err) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?} : {:?}", key, s, err.message),
                }
            }
        }
    }

    pub fn settlementCurrencyFxRate(riskFactorModel: &RiskFactorModel, model: &ContractModel, time: &IsoDatetime, state: &StateSpace) -> f64{    
        let settlement_currency = model.settlement_currency.clone();
        let currency = model.currency.clone();
        
        if settlement_currency.is_none()  || currency == Some(settlement_currency.clone().unwrap().to_currency()) {
            1.0
        }
        else {
            let strings = vec![currency.unwrap(), settlement_currency.clone().unwrap().to_currency()]; // refaire plus proprement pour pas melanger Currency et setllment currency

            let str_slices: Vec<String> = strings.iter().map(|s| s.value()).collect();
            let joined = str_slices.join(" ");

            riskFactorModel.state_at(&joined, time, state, model,true).unwrap()
        }
        
    }
}