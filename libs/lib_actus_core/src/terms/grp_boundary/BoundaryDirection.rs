use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_boundary::boundary_direction::DECR::DECR;
use crate::terms::grp_boundary::boundary_direction::INCR::INCR;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::util::Value::Value;

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

    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => BoundaryDirection::from_str(n),
            None => Ok(BoundaryDirection::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
}

impl FromStr for BoundaryDirection {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "INCR" => Ok(Self::INCR(INCR::new())),
            "DECR" => Ok(Self::DECR(DECR::new())),
            _ => Err(ParseError { message: format!("Invalid BoundaryDirection: {}", s) })
        }
    }
}

impl Default for BoundaryDirection {
    fn default() -> Self {
        Self::None
    }
}

