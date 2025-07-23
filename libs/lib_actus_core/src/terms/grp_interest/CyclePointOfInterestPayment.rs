use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::terms::grp_interest::cycle_point_of_interest_payment::B::B;
use crate::terms::grp_interest::cycle_point_of_interest_payment::E::E;

use crate::types::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CyclePointOfInterestPayment {
    B(B),
    E(E),
}

impl CyclePointOfInterestPayment {


    pub fn new(element: &str) -> Result<Self, String> {
        CyclePointOfInterestPayment::from_str(element)
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        //cu::provide(string_map, key)

        crate::util::ProvideFuncs::provide(string_map, key)
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

impl FromStr for CyclePointOfInterestPayment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B" => Ok(CyclePointOfInterestPayment::B(B)),
            "E" => Ok(CyclePointOfInterestPayment::E(E)),
            _ => Err(format!("Invalid CyclePointOfInterestPayment: {}", s))
        }
    }
}

impl Default for CyclePointOfInterestPayment {
    fn default() -> Self {
        CyclePointOfInterestPayment::E(E)
    }
}

impl fmt::Display for CyclePointOfInterestPayment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::B(B) => write!(f, "FeeBasis: {}", B.to_string()),
            Self::E(E) => write!(f, "FeeBasis: {}", E.to_string()),
        }
    }
}
