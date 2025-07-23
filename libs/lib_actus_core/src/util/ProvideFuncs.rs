use std::collections::HashMap;
use std::str::FromStr;
use crate::types::Value::Value;

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
    string_map
        .get(key)
        .and_then(|s| {
            Some(vec![s.to_string()])
        })
        .map(|b| b) // On stocke la convention dans une Box
    //.unwrap_or_default()
}

pub fn provide_f64_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<f64>> {

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
    T: FromStr<Err = String> + Default,
{
    match string_map.get(key) {
        None => Some(T::default()), // Clé absente : valeur par défaut dans un Some
        Some(s) => {
            match T::from_str(s.as_string().unwrap().as_str()) {
                Ok(value) => Some(value), // Valeur valide
                Err(err) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?} : {:?}", key, s, "test".to_string()),
            }
        }
    }
}
