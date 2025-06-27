use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_optionality::penalty_type::N::N;
use crate::terms::grp_optionality::penalty_type::A::A;
use crate::terms::grp_optionality::penalty_type::R::R;
use crate::terms::grp_optionality::penalty_type::I::I;
use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PenaltyType {
    N(N),
    A(A),
    R(R),
    I(I),
}

impl PenaltyType {
    pub fn description(&self) -> String {
        match self {
            Self::N(N) => N.type_str(),
            Self::A(A) => A.type_str(),
            Self::R(R) => R.type_str(),
            Self::I(I) => I.type_str(),
        }
    }
    pub fn new_N() -> Self {
        PenaltyType::N(N::new())
    }
    pub fn new_A() -> Self {
        PenaltyType::A(A::new())
    }
    pub fn new_R() -> Self {
        PenaltyType::R(R::new())
    }
    pub fn new_I() -> Self {
        PenaltyType::I(I::new())
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        cu::provide(string_map, key)
    }
}

impl FromStr for PenaltyType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Self::new_N()),
            "A" => Ok(Self::new_A()),
            "R" => Ok(Self::new_R()),
            "I" => Ok(Self::new_I()),
            _ => Err(ParseError { message: format!("Invalid PenaltyType {}", s)})
        }
    }
}

impl Default for PenaltyType {
    fn default() -> Self {
        Self::N(N)
    }
}

