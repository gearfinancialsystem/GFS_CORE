use std::fmt;
use crate::terms::grp_notional_principal::scaling_effect::Ono::ONO;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct OOO;

impl OOO {
    pub fn new() -> Self {
        return OOO;
    }
    pub fn type_str(&self) -> String {
        return "OOO Scaling Effect".to_string();
    }
}

impl fmt::Display for OOO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OOO")
    }
}