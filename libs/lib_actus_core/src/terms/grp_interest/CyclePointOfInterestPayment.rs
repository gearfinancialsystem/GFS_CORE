use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_interest::cycle_point_of_interest_payment::B::B;
use crate::terms::grp_interest::cycle_point_of_interest_payment::E::E;
use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CyclePointOfInterestPayment {
    B(B),
    E(E),
}

impl CyclePointOfInterestPayment {
    pub fn description(&self) -> String {
        match self {
            CyclePointOfInterestPayment::B(B) => B.type_str(),
            CyclePointOfInterestPayment::E(E) => E.type_str(),
        }
    }

    pub fn new(element: &str) -> Result<Self, ParseError> {
        CyclePointOfInterestPayment::from_str(element)
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        cu::provide(string_map, key)
    }
}

impl FromStr for CyclePointOfInterestPayment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B" => Ok(CyclePointOfInterestPayment::B(B)),
            "E" => Ok(CyclePointOfInterestPayment::E(E)),
            _ => Err(ParseError { message: format!("Invalid CyclePointOfInterestPayment: {}", s)})
        }
    }
}

impl Default for CyclePointOfInterestPayment {
    fn default() -> Self {
        CyclePointOfInterestPayment::E(E)
    }
}

