use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::CP::CP;
use crate::terms::grp_optionality::option_type::P::P;
use crate::exceptions::ParseError::ParseError;
use crate::util::Value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionType {
    C(C),
    P(P),
    CP(CP)
}

impl OptionType {
    
    pub fn new(element: &str) -> Result<Self, ParseError> {
        OptionType::from_str(element)
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

impl FromStr for OptionType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "C" => Ok(Self::C(C::new())),
            "P" => Ok(Self::P(P::new())),
            "CP" => Ok(Self::CP(CP::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for OptionType {
    fn default() -> Self {
        Self::C(C::new())
    }
}
impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::C(C) => write!(f, "OptionType: {}", C.to_string()),
            Self::P(P) => write!(f, "OptionType: {}", P.to_string()),
            Self::CP(CP) => write!(f, "OptionType: {}", CP.to_string()),
        }
    }
}
