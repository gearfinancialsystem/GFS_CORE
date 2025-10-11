use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_optionality::prepayment_effect::A::A;
use crate::terms::grp_optionality::prepayment_effect::M::M;
use crate::terms::grp_optionality::prepayment_effect::N::N;

use lib_actus_types::types::Value::Value;

#[derive(PartialEq, Eq)]
pub enum PrepaymentEffect {
    N(N),
    A(A),
    M(M)
}

impl PrepaymentEffect {
    
    pub fn new(element: &str) -> Result<Self, String> {
        PrepaymentEffect::from_str(element)
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

impl FromStr for PrepaymentEffect {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Self::N(N::new())),
            "A" => Ok(Self::A(A::new())),
            "M" => Ok(Self::M(M::new())),
            _ => Err(format!("Invalid BusinessDayAdjuster: {}", s))
        }
    }
}

impl Default for PrepaymentEffect {
    fn default() -> Self {
        Self::N(N::new())
    }
}

impl fmt::Display for PrepaymentEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::N(v) => write!(f, "PrepaymentEffect: {}", v.to_string()),
            Self::A(v) => write!(f, "PrepaymentEffect: {}", v.to_string()),
            Self::M(v) => write!(f, "PrepaymentEffect: {}", v.to_string()),

        }
    }
}


