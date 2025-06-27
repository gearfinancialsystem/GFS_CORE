use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::CP::CP;
use crate::terms::grp_optionality::option_type::P::P;
use crate::exceptions::ParseError::ParseError;
use crate::util::Value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
}

impl FromStr for OptionType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "C" => Ok(Self::new_C()),
            "P" => Ok(Self::new_P()),
            "CP" => Ok(Self::new_CP()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for OptionType {
    fn default() -> Self {
        Self::new_C()
    }
}

