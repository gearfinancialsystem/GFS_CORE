use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_boundary::boundary_direction::DECR::DECR;
use crate::terms::grp_boundary::boundary_direction::INCR::INCR;
use crate::types::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BoundaryDirection {
    INCR(INCR),
    DECR(DECR),
    None
}

impl BoundaryDirection {
    pub fn description(&self) -> String {
        match self {
            Self::INCR(INCR) => DECR.type_str(),
            Self::DECR(DECR) => DECR.type_str(),
            Self::None => "".to_string()
        }
    }

    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => BoundaryDirection::from_str(n),
            None => Ok(BoundaryDirection::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        //crate::utils::ProvideFuncs::provide(string_map, key)
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

impl FromStr for BoundaryDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INCR" => Ok(Self::INCR(INCR::new())),
            "DECR" => Ok(Self::DECR(DECR::new())),
            _ => Err("te".to_string()) //Err(String { message: format!("Invalid BoundaryDirection: {}", s) })
        }
    }
}

impl Default for BoundaryDirection {
    fn default() -> Self {
        Self::None
    }
}

