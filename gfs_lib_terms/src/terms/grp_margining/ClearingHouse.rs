use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_margining::clearing_house::N::N;
use crate::terms::grp_margining::clearing_house::Y::Y;

use gfs_lib_types::types::Value::Value;

#[derive(PartialEq, Eq)]
pub enum ClearingHouse {
    Y(Y),
    N(N),
    None
}

impl ClearingHouse {
    
    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => ClearingHouse::from_str(n),
            None => Ok(ClearingHouse::None),
        }
    }
    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
}


impl Default for ClearingHouse {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for ClearingHouse {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "Y" => Ok(Self::Y(Y::new())),
            "N" => Ok(Self::N(N::new())),
            _ => Err(format!("Invalid BusinessDayAdjuster: {}", s))
        }
    }
}

impl fmt::Display for ClearingHouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Y(Y) => write!(f, "ClearingHouse: {}", Y.to_string()),
            Self::N(N) => write!(f, "ClearingHouse: {}", N.to_string()),
            Self::None => write!(f, "ClearingHouse: None"),
        }
    }
}


