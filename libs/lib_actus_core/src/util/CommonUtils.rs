use std::collections::HashMap;
use std::str::FromStr;
use crate::attributes::ContractModel::ContractModel;
use crate::util::Value::Value;
use crate::exceptions::ParseError::ParseError;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::types::isoDatetime::IsoDatetime;

pub struct CommonUtils;


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
        let settlementCurrency = model.settlementCurrency.clone();
        let currency = model.currency.clone();
        
        if settlementCurrency.is_none()  || currency == settlementCurrency {
            1.0
        }
        else {
            let strings = vec![currency.unwrap(), settlementCurrency.unwrap()];

            let str_slices: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
            let joined = str_slices.join(" ");

            riskFactorModel.state_at(&joined, time, state, model,true).unwrap()
        }
        
    }
}