use std::collections::HashMap;
use std::str::FromStr;
use crate::util::Value::Value;
use crate::exceptions::ParseError::ParseError;
// use crate::terms::grp_settlement::DeliverySettlement::S;





pub struct CommonUtils;


impl CommonUtils {

    // pub fn provide_box_f64(string_map: &HashMap<String, String>, key: &str) -> Box<Option<f64>>{
    //     string_map.get(key).cloned().map(|c| Box::new(c.parse::<f64>().ok())).unwrap_or_else(|| Box::new(None))
    // }
    pub fn provide_box_f64(string_map: &HashMap<String, Value>, key: &str) -> Option<Box<f64>> {
        //string_map.get(key).and_then(|s| s.parse::<f64>().ok()).map(Box::new)
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match <f64>::from_str(s.as_string()?.as_str()) {
                    Ok(value) => Some(Box::new(value)), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
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
    pub fn provide_f64default(string_map: &HashMap<String, Value>, key: &str, default: f64) -> Option<f64>
    {
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
    // pub fn provide_box_string(string_map: &HashMap<String, String>, key: &str) -> Box<Option<String>> {
    //     string_map.get(key).cloned().map(|c| Box::new(Some(c))).unwrap_or_else(|| Box::new(None))
    // }
    pub fn provide_box_string(string_map: &HashMap<String, Value>, key: &str) -> Option<Box<String>> {
        string_map.get(key).unwrap().to_string().map(Box::new)
    }
    pub fn provide_string(string_map: &HashMap<String, Value>, key: &str) -> Option<String> {
        println!("{:?}", key);
        //string_map.get(key).unwrap().extract_string()


        string_map
            .get(key)
            .and_then(|s| {
                s.extract_string()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
    pub fn provide_string_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<String>> {
        println!("{:?}", key);
        //string_map.get(key).unwrap().extract_string()


        string_map
            .get(key)
            .and_then(|s| {
                s.extract_vec_str()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
    pub fn provide_f64_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<f64>> {
        println!("{:?}", key);
        //string_map.get(key).unwrap().extract_string()


        string_map
            .get(key)
            .and_then(|s| {
                s.extract_vec_f64()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
    

    // Fonction générique provide
    pub fn provide<T>(string_map: &HashMap<String, Value>, key: &str) -> Option<T>
    where
        T: FromStr<Err = ParseError> + Default,
    {
        match string_map.get(key) {
            None => Some(T::default()), // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match T::from_str(s.extract_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(err) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?} : {:?}", key, s, err.message),
                }
            }
        }
    }
    // pub fn is_none(value: &Option<AnyBox>) -> bool {
    //     match value {
    //         None => true,
    //         Some(v) => false,
    //     }
    // }
    // pub fn is_none_string(value: &Option<&String>) -> bool {
    //     match value {
    //         None => true,
    //         Some(v) => false,
    //     }
    // }

    // pub fn settlement_currency_fx_rate(
    //     risk_factor_model: &dyn RiskFactorModelTrait,
    //     model: &dyn ContractModelTrait,
    //     time: IsoDatetime,
    //     state: &StateSpace,
    // ) -> f64 {
    //     let settlement_currency = model.get_as("SettlementCurrency");
    //     let currency = model.get_as("Currency");

    //     let are_equal = match(settlement_currency, currency){
    //         (Some(a), Some(b)) => a.downcast_ref::<&str>() == b.downcast_ref::<&str>(), _ => false,
    //     };

    //     if CommonUtils::is_none(settlement_currency) || are_equal {
    //         1.0
    //     } else {
    //         let currency_pair = format!("{:?}/{:?}", currency.unwrap(), settlement_currency.unwrap());
    //         risk_factor_model.state_at(&currency_pair, &time, state, model)
    //     }
    // }
}