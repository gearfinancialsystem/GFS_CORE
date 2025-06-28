use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_margining::clearing_house::N::N;
use crate::terms::grp_margining::clearing_house::Y::Y;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;

#[derive(PartialEq, Eq)]
pub enum ClearingHouse {
    Y(Y),
    N(N),
    None
}

impl ClearingHouse {
    pub fn description(&self) -> String {
        match self {
            Self::Y(Y) => Y.type_str(),
            Self::N(N) => N.type_str(),
            Self::None => "".to_string(),
        }
    }
    
    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => ClearingHouse::from_str(n),
            None => Ok(ClearingHouse::None),
        }
    }
}


impl Default for ClearingHouse {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for ClearingHouse {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "Y" => Ok(Self::Y(Y::new())),
            "N" => Ok(Self::N(N::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}




