use std::fmt;
use crate::terms::grp_optionality::prepayment_effect::M::M;

#[derive(Debug, Eq, PartialEq)]

pub struct N;

impl N {
    pub fn new() -> Self {
        return N;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "N")
    }
}