use std::fmt;
use crate::terms::grp_notional_principal::unitx::CUU::CUU;

#[derive(Debug, Eq, PartialEq)]

pub struct GLN;

impl GLN {
    pub fn new() -> Self {
        return GLN;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
impl fmt::Display for GLN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GLN")
    }
}