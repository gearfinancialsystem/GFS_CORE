use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;

use crate::terms::grp_boundary::boundary_effect::Infil::INFIL;
use crate::terms::grp_boundary::boundary_effect::Insel::INSEL;
use crate::terms::grp_boundary::boundary_effect::Out::OUT;


#[derive(PartialEq, Eq, Debug)]
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
    pub fn new_INFIL() -> Self {
        BoundaryEffect::INFIL(INFIL::new())
    }
    pub fn new_INSEL() -> Self {
        BoundaryEffect::INSEL(INSEL::new())
    }
    pub fn new_OUT() -> Self {
        BoundaryEffect::OUT(OUT::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                BoundaryEffect::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for BoundaryEffect {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INFIL" => Ok(BoundaryEffect::new_INFIL()),
            "INSEL" => Ok(BoundaryEffect::new_INSEL()),
            "OUT" => Ok(BoundaryEffect::new_OUT()),
            _ => Err(ParseError { message: format!("Invalid BoundaryEffect: {}", s)})
        }
    }
}

impl Default for BoundaryEffect {
    fn default() -> Self {
        Self::None
    }
}


