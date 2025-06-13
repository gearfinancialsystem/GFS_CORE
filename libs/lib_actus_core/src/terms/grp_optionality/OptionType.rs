use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::CP::CP;
use crate::terms::grp_optionality::option_type::P::P;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::exceptions::ParseError::ParseError;


#[derive(Debug, PartialEq, Eq)]
pub enum OptionType {
    C(C),
    P(P),
    CP(CP)
}

impl OptionType {
    pub fn description(&self) -> String {
        match self {
            Self::C(C) => C.type_str(),
            Self::P(P) => P.type_str(),
            Self::CP(CP) => CP.type_str(),
        }
    }
    pub fn new_C() -> Self {
        Self::C(C::new())
    }
    pub fn new_P() -> Self {
        Self::P(P::new())
    }
    pub fn new_CP() -> Self {
        Self::CP(CP::new())
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

impl FromStr for OptionType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "C" => Ok(Self::new_C()),
            "P" => Ok(Self::new_P()),
            "CP" => Ok(Self::new_CP()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for OptionType {
    fn default() -> Self {
        Self::new_C()
    }
}

