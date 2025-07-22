use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_counterparty::seniority::J::J;
use crate::terms::grp_counterparty::seniority::S::S;
use crate::exceptions::ParseError::ParseError;
use crate::util::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Seniority {
    S(S),
    J(J),
    None
}

impl Seniority {

    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => Seniority::from_str(n),
            None => Ok(Seniority::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
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

impl FromStr for Seniority {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "S" => Ok(Self::S(S::new())),
            "J" => Ok(Self::J(J::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for Seniority {
    fn default() -> Self {
        Seniority::None
    }
}
impl fmt::Display for Seniority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::S(S) => write!(f, "Seniority: {}", S.to_string()),
            Self::J(J) => write!(f, "Seniority: {}", J.to_string()),
            Self::None => write!(f, "GuaranteedExposure: No value was given")
        }
    }
}
