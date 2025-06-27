use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;

use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::E::E;
use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CyclePointOfRateReset {
    B(B),
    E(E),
}

impl CyclePointOfRateReset {
    pub fn description(&self) -> String {
        match self {
            CyclePointOfRateReset::B(B) => B.type_str(),
            CyclePointOfRateReset::E(E) => E.type_str(),
        }
    }
    pub fn new_B() -> Self {
        Self::B(B::new())
    }
    pub fn new_E() -> Self {
        Self::E(E::new())
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        cu::provide(string_map, key)
    }
}

impl FromStr for CyclePointOfRateReset {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B" => Ok(Self::new_B()),
            "E" => Ok(Self::new_E()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for CyclePointOfRateReset {
    fn default() -> Self {
        Self::B(B)
    }
}

