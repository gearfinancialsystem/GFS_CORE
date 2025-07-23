use std::collections::HashMap;
use std::str::FromStr;

use crate::terms::grp_boundary::boundary_effect::Infil::INFIL;
use crate::terms::grp_boundary::boundary_effect::Insel::INSEL;
use crate::terms::grp_boundary::boundary_effect::Out::OUT;
use crate::types::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BoundaryEffect {
    INFIL(INFIL),
    INSEL(INSEL),
    OUT(OUT),
    None
}

impl BoundaryEffect {
    pub fn description(&self) -> String {
        match self {
            Self::INFIL(INFIL) => INFIL.type_str(),
            Self::INSEL(INSEL) => INSEL.type_str(),
            Self::OUT(OUT) => OUT.type_str(),
            Self::None => "None".to_string(),
        }
    }

    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => BoundaryEffect::from_str(n),
            None => Ok(BoundaryEffect::None),
        }
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        // crate::utils::ProvideFuncs::provide(string_map, key)
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

impl FromStr for BoundaryEffect {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INFIL" => Ok(BoundaryEffect::INFIL(INFIL::new())),
            "INSEL" => Ok(BoundaryEffect::INSEL(INSEL::new())),
            "OUT" => Ok(BoundaryEffect::OUT(OUT::new())),
            _ => Err(format!("Invalid BoundaryEffect: {}", s) )
        }
    }
}

impl Default for BoundaryEffect {
    fn default() -> Self {
        Self::None
    }
}


