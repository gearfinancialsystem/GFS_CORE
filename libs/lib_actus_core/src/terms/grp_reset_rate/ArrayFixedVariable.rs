use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::E::E;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::terms::grp_reset_rate::fixed_variable::F::F;
use crate::terms::grp_reset_rate::fixed_variable::V::V;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::util::ParseError::ParseError;

#[derive(Debug, Eq, PartialEq)]
pub enum ArrayFixedVariable {
    F(F),
    V(V),
    None
}

impl ArrayFixedVariable {
    pub fn description(&self) -> String {
        match self {
            Self::F(F) => F.type_str(),
            Self::V(V) => V.type_str(),
            Self::None => "None".to_string(),
        }
    }
    pub fn new_F() -> Self {
        Self::F(F::new())
    }
    pub fn new_V() -> Self {
        Self::V(V::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                ArrayFixedVariable::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for ArrayFixedVariable {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "F" => Ok(Self::new_F()),
            "V" => Ok(Self::new_V()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for ArrayFixedVariable {
    fn default() -> Self {
        Self::None
    }
}


