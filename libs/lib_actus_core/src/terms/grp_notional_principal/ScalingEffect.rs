use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_margining::ClearingHouse::ClearingHouse;
use crate::terms::grp_notional_principal::scaling_effect::Ooo::OOO;
use crate::terms::grp_notional_principal::scaling_effect::Ono::ONO;
use crate::terms::grp_notional_principal::scaling_effect::Ioo::IOO;
use crate::terms::grp_notional_principal::scaling_effect::Ino::INO;

use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScalingEffect {
    OOO(OOO),
    IOO(IOO),
    ONO(ONO),
    INO(INO),
}

impl ScalingEffect {

    
    pub fn new(element: &str) -> Result<Self, ParseError> {
        ScalingEffect::from_str(element)
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
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

impl FromStr for ScalingEffect {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "OOO" => Ok(ScalingEffect::OOO(OOO::new())),
            "IOO" => Ok(ScalingEffect::IOO(IOO::new())),
            "ONO" => Ok(ScalingEffect::ONO(ONO::new())),
            "INO" => Ok(ScalingEffect::INO(INO::new())),
            _ => Err(ParseError { message: format!("Invalid ScalingEffect: {}", s)})
        }
    }
}

impl Default for ScalingEffect {
    fn default() -> Self {
        ScalingEffect::OOO(OOO)
    }
}

impl fmt::Display for ScalingEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OOO(OOO) => write!(f, "ScalingEffect: {}", OOO.to_string()),
            Self::IOO(IOO) => write!(f, "ScalingEffect: {}", IOO.to_string()),
            Self::ONO(ONO) => write!(f, "ScalingEffect: {}", ONO.to_string()),
            Self::INO(INO) => write!(f, "ScalingEffect: {}", INO.to_string()),
        }
    }
}