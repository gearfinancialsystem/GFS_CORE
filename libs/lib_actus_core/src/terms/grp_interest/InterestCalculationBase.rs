use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntied::NTIED;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::types::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InterestCalculationBase {
    NT(NT),
    NTIED(NTIED),
    NTL(NTL)
}

impl InterestCalculationBase {
    pub fn new_NT() -> Self {
        InterestCalculationBase::NT(NT::new())
    }
    pub fn new_NTIED() -> Self {
        InterestCalculationBase::NTIED(NTIED::new())
    }
    pub fn new_NTL() -> Self {
        InterestCalculationBase::NTL(NTL::new())
    }

    pub fn new(element: &str) -> Result<Self, String> {
        InterestCalculationBase::from_str(element)
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
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

impl Default for InterestCalculationBase {
    fn default() -> Self {
        InterestCalculationBase::NT(NT::new())
    }
}

impl FromStr for InterestCalculationBase {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NT" => Ok(InterestCalculationBase::NT(NT::new())),
            "NTIED" => Ok(InterestCalculationBase::NTIED(NTIED::new())),
            "NTL" => Ok(InterestCalculationBase::NTL(NTL::new())),
            _ => Err(format!("Invalid BusinessDayAdjuster"))
        }
    }
}


impl fmt::Display for InterestCalculationBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NT(NT) => write!(f, "InterestCalculationBase: {}", NT.to_string()),
            Self::NTIED(NTIED) => write!(f, "InterestCalculationBase: {}", NTIED.to_string()),
            Self::NTL(NTL) => write!(f, "InterestCalculationBase: {}", NTL.to_string()),
        }
    }
}