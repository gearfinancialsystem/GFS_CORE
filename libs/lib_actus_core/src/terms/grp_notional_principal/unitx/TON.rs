use std::fmt;
use crate::terms::grp_notional_principal::unitx::STN::STN;

#[derive(Debug, Eq, PartialEq)]

pub struct TON;

impl TON {
    pub fn new() -> Self {
        return TON;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl fmt::Display for TON {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TON")
    }
}