use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_optionality::prepayment_effect::A::A;
use crate::terms::grp_optionality::prepayment_effect::M::M;
use crate::terms::grp_optionality::prepayment_effect::N::N;
use crate::exceptions::ParseError::ParseError;


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
    pub fn new_N() -> Self {
        Self::N(N::new())
    }
    pub fn new_A() -> Self {
        Self::A(A::new())
    }
    pub fn new_M() -> Self {
        Self::M(M::new())
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

impl FromStr for PrepaymentEffect {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Self::new_N()),
            "A" => Ok(Self::new_A()),
            "M" => Ok(Self::new_M()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl Default for PrepaymentEffect {
    fn default() -> Self {
        Self::new_N()
    }
}

