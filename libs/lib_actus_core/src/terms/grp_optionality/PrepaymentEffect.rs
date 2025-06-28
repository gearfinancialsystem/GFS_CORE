use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::prepayment_effect::A::A;
use crate::terms::grp_optionality::prepayment_effect::M::M;
use crate::terms::grp_optionality::prepayment_effect::N::N;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_optionality::OptionType::OptionType;

#[derive(PartialEq, Eq)]
pub enum PrepaymentEffect {
    N(N),
    A(A),
    M(M)
}

impl PrepaymentEffect {
    pub fn description(&self) -> String {
        match self {
            PrepaymentEffect::N(N) => N.type_str(),
            PrepaymentEffect::A(A) => A.type_str(),
            PrepaymentEffect::M(M) => M.type_str(),
        }
    }

    pub fn new(element: &str) -> Result<Self, ParseError> {
        PrepaymentEffect::from_str(element)
    }
}

impl FromStr for PrepaymentEffect {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Self::N(N::new())),
            "A" => Ok(Self::A(A::new())),
            "M" => Ok(Self::M(M::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for PrepaymentEffect {
    fn default() -> Self {
        Self::N(N::new())
    }
}

