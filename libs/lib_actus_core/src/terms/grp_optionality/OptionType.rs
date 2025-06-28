use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::CP::CP;
use crate::terms::grp_optionality::option_type::P::P;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
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

    pub fn new(element: &str) -> Result<Self, ParseError> {
        OptionType::from_str(element)
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
}

impl FromStr for OptionType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "C" => Ok(Self::C(C::new())),
            "P" => Ok(Self::P(P::new())),
            "CP" => Ok(Self::CP(CP::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for OptionType {
    fn default() -> Self {
        Self::C(C::new())
    }
}

