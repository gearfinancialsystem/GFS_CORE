use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::CP::CP;
use crate::terms::grp_optionality::option_type::P::P;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_optionality::penalty_type::N::N;
use crate::terms::grp_optionality::penalty_type::A::A;
use crate::terms::grp_optionality::penalty_type::R::R;
use crate::terms::grp_optionality::penalty_type::I::I;
use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PenaltyType {
    N(N),
    A(A),
    R(R),
    I(I),
}

impl PenaltyType {


    pub fn new(element: &str) -> Result<Self, ParseError> {
        PenaltyType::from_str(element)
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        cu::provide(string_map, key)
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

impl FromStr for PenaltyType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Self::N(N::new())),
            "A" => Ok(Self::A(A::new())),
            "R" => Ok(Self::R(R::new())),
            "I" => Ok(Self::I(I::new())),
            _ => Err(ParseError { message: format!("Invalid PenaltyType {}", s)})
        }
    }
}

impl Default for PenaltyType {
    fn default() -> Self {
        Self::N(N)
    }
}

impl fmt::Display for PenaltyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::N(N) => write!(f, "PenaltyType: {}", N.to_string()),
            Self::A(A) => write!(f, "PenaltyType: {}", A.to_string()),
            Self::R(R) => write!(f, "PenaltyType: {}", R.to_string()),
            Self::I(I) => write!(f, "PenaltyType: {}", I.to_string()),
        }
    }
}
