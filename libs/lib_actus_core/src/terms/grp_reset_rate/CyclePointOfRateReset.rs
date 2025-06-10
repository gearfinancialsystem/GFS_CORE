use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;

use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::E::E;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Eq, PartialEq)]
pub enum CyclePointOfRateReset {
    B(B),
    E(E),
    None
}

impl CyclePointOfRateReset {
    pub fn description(&self) -> String {
        match self {
            CyclePointOfRateReset::B(B) => B.type_str(),
            CyclePointOfRateReset::E(E) => E.type_str(),
            CyclePointOfRateReset::None => "None".to_string(),
        }
    }
    pub fn new_B() -> Self {
        Self::B(B::new())
    }
    pub fn new_E() -> Self {
        Self::E(E::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                CyclePointOfRateReset::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for CyclePointOfRateReset {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B" => Ok(Self::new_B()),
            "E" => Ok(Self::new_E()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for CyclePointOfRateReset {
    fn default() -> Self {
        Self::B(B)
    }
}

