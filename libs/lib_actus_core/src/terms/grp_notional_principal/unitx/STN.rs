use std::fmt;
use crate::terms::grp_notional_principal::unitx::PND::PND;

#[derive(Debug, Eq, PartialEq)]

pub struct STN;

impl STN {
    pub fn new() -> Self {
        return STN;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl fmt::Display for STN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STN")
    }
}