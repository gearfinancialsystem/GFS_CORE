use std::{collections::HashMap, str::FromStr};
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_fees::fee_basis::N::N;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_counterparty::Seniority::Seniority;
use crate::util::Value::Value;

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

    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => FeeBasis::from_str(n),
            None => Ok(FeeBasis::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s.as_string().unwrap().as_str()).ok()
            })
            .map(|b|b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
}


impl FromStr for FeeBasis {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::A(A::new())),
            "N" => Ok(Self::N(N::new())),
            _ => Err(ParseError { message: format!("Invalid FeeBasis: {}", s)})
        }
    }
}

impl Default for FeeBasis {
    fn default() -> Self {
        FeeBasis::None
    }
}

