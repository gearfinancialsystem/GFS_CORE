use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_boundary::boundary_leg_initially_active::FIL::FIL;
use crate::terms::grp_boundary::boundary_leg_initially_active::SEL::SEL;
use crate::exceptions::ParseError::ParseError;
use crate::util::CommonUtils::Value;

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
    pub fn new_FIL() -> Self {
        Self::FIL(FIL::new())
    }
    pub fn new_SEL() -> Self {
        Self::SEL(SEL::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
    pub fn to_stringx(&self) -> Result<String, ParseError> {
        match self {
            Self::FIL(FIL) => Ok("FIL".to_string()),
            Self::SEL(SEL) => Ok("SEL".to_string()),
            _ => Err(ParseError { message: format!("Invalid TOSTRING ContractPerformance ")})
        }

    }
}

impl FromStr for BoundaryLegInitiallyActive {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FIL" => Ok(Self::new_FIL()),
            "SEL" => Ok(Self::new_SEL()),
            "NUll" => Ok(Self::None),
            _ => Err(ParseError { message: format!("Invalid BoundaryLegInitiallyActive: {}", s)})
        }
    }
}

impl Default for BoundaryLegInitiallyActive {
    fn default() -> Self {
        Self::None
    }
}



