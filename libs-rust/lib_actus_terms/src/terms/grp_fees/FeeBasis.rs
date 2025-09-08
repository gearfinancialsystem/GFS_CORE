use std::{collections::HashMap, fmt, str::FromStr};
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_fees::fee_basis::N::N;

use lib_actus_types::types::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FeeBasis {
    A(A),
    N(N),
    None
}

impl FeeBasis {

    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => FeeBasis::from_str(n),
            None => Ok(FeeBasis::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s.as_string().unwrap().as_str()).ok()
            })
            .map(|b|b) // On stocke la convention dans une Box
        //.unwrap_or_default()
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


impl FromStr for FeeBasis {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::A(A::new())),
            "N" => Ok(Self::N(N::new())),
            _ => Err(format!("Invalid FeeBasis:"))
        }
    }
}

impl Default for FeeBasis {
    fn default() -> Self {
        FeeBasis::None
    }
}
impl fmt::Display for FeeBasis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::A(A) => write!(f, "FeeBasis: {}", A.to_string()),
            Self::N(N) => write!(f, "FeeBasis: {}", N.to_string()),
            Self::None => write!(f, "GuaranteedExposure: No value was given")
        }
    }
}

