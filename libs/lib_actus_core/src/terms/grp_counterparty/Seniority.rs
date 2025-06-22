use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::seniority::J::J;
use crate::terms::grp_counterparty::seniority::S::S;
use crate::exceptions::ParseError::ParseError;


#[derive(PartialEq, Eq, Debug)]
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
    pub fn new_S() -> Self {
        Self::S(S::new())
    }
    pub fn new_J() -> Self {
        Self::J(J::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
}

impl FromStr for Seniority {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "S" => Ok(Self::new_S()),
            "J" => Ok(Self::new_J()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for Seniority {
    fn default() -> Self {
        Seniority::None
    }
}

