use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_boundary::boundary_leg_initially_active::FIL::FIL;
use crate::terms::grp_boundary::boundary_leg_initially_active::SEL::SEL;

use lib_actus_types::types::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BoundaryLegInitiallyActive {
    FIL(FIL),
    SEL(SEL),
    None
}

impl BoundaryLegInitiallyActive {
    pub fn description(&self) -> String {
        match self {
            Self::FIL(FIL) => FIL.type_str(),
            Self::SEL(SEL) => SEL.type_str(),
            Self::None => "".to_string()
        }
    }

    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => BoundaryLegInitiallyActive::from_str(n),
            None => Ok(BoundaryLegInitiallyActive::None),
        }
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::utils::ProvideFuncs::provide(string_map, key)
    }
    pub fn to_stringx(&self) -> Result<String, String> {
        match self {
            Self::FIL(FIL) => Ok("FIL".to_string()),
            Self::SEL(SEL) => Ok("SEL".to_string()),
            Self::SEL(SEL) => Ok("SEL".to_string()),
            _ => Err(format!("Invalid TOSTRING ContractPerformance "))
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

impl FromStr for BoundaryLegInitiallyActive {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FIL" => Ok(Self::FIL(FIL::new())),
            "SEL" => Ok(Self::SEL(SEL::new())),
            _ => Err(format!("Invalid BoundaryLegInitiallyActive: {}", s))
        }
    }
}

impl Default for BoundaryLegInitiallyActive {
    fn default() -> Self {
        Self::None
    }
}



