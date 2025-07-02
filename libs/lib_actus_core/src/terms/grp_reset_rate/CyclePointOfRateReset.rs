use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_optionality::penalty_type::A::A;
use crate::terms::grp_optionality::penalty_type::I::I;
use crate::terms::grp_optionality::penalty_type::N::N;
use crate::terms::grp_optionality::penalty_type::R::R;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::E::E;
use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CyclePointOfRateReset {
    B(B),
    E(E),
}

impl CyclePointOfRateReset {

    pub fn new(element: &str) -> Result<Self, ParseError> {
        CyclePointOfRateReset::from_str(element)
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

impl FromStr for CyclePointOfRateReset {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B" => Ok(Self::B(B::new())),
            "E" => Ok(Self::E(E::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for CyclePointOfRateReset {
    fn default() -> Self {
        Self::B(B)
    }
}
impl fmt::Display for CyclePointOfRateReset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::B(B) => write!(f, "PenaltyType: {}", B.to_string()),
            Self::E(E) => write!(f, "PenaltyType: {}", E.to_string()),
        }
    }
}

