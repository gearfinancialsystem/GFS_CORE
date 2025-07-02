use std::fmt;
use crate::terms::grp_notional_principal::unitx::BSH::BSH;

#[derive(Debug, Eq, PartialEq)]

pub struct CUU;

impl CUU {
    pub fn new() -> Self {
        return CUU;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl fmt::Display for CUU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CUU")
    }
}