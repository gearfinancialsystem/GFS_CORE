use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::seniority::J::J;
use crate::terms::grp_counterparty::seniority::S::S;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_counterparty::GuaranteedExposure::GuaranteedExposure;
use crate::util::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Seniority {
    S(S),
    J(J),
    None
}

impl Seniority {
    pub fn description(&self) -> String {
        match self {
            Self::S(S) => S.type_str(),
            Self::J(J) => J.type_str(),
            Self::None => "".to_string()
        }
    }

    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => Seniority::from_str(n),
            None => Ok(Seniority::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
}

impl FromStr for Seniority {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "S" => Ok(Self::S(S::new())),
            "J" => Ok(Self::J(J::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for Seniority {
    fn default() -> Self {
        Seniority::None
    }
}

