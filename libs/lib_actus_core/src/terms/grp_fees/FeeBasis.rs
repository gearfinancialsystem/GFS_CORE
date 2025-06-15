use std::{collections::HashMap, str::FromStr};
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_fees::fee_basis::N::N;
use crate::terms::grp_margining::ClearingHouse::ClearingHouse;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::exceptions::ParseError::ParseError;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FeeBasis {
    A(A),
    N(N),
    None
}

impl FeeBasis {
    pub fn description(&self) -> String {
        match self {
            Self::A(A) => A.type_str(),
            Self::N(N) => N.type_str(),
            Self::None => "".to_string()
        }
    }
    pub fn new_A() -> Self {
        Self::A(A::new())
    }
    pub fn new_N() -> Self {
        Self::N(N::new())
    }
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Option<Box<Self>> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            //.unwrap_or_default()
    }
    pub fn provide(string_map: &HashMap<String, String>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b|b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
}


impl FromStr for FeeBasis {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::new_A()),
            "N" => Ok(Self::new_N()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for FeeBasis {
    fn default() -> Self {
        FeeBasis::None
    }
}

