use std::collections::HashMap;
use std::str::FromStr;
use crate::util::ParseError::ParseError;
use crate::terms::grp_interest::cycle_point_of_interest_payment::B::B;
use crate::terms::grp_interest::cycle_point_of_interest_payment::E::E;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Eq, PartialEq)]
pub enum CyclePointOfInterestPayment {
    B(B),
    E(E),
    None
}

impl CyclePointOfInterestPayment {
    pub fn description(&self) -> String {
        match self {
            CyclePointOfInterestPayment::B(B) => B.type_str(),
            CyclePointOfInterestPayment::E(E) => E.type_str(),
            CyclePointOfInterestPayment::None => "None".to_string(),
        }
    }
    pub fn new_B() -> Self {
        CyclePointOfInterestPayment::B(B::new())
    }
    pub fn new_RPL() -> Self {
        CyclePointOfInterestPayment::E(E::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                CyclePointOfInterestPayment::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for CyclePointOfInterestPayment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "B" => Ok(CyclePointOfInterestPayment::B(B)),
            "E" => Ok(CyclePointOfInterestPayment::E(E)),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for CyclePointOfInterestPayment {
    fn default() -> Self {
        CyclePointOfInterestPayment::E(E)
    }
}

