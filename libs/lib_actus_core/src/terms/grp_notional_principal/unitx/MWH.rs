use std::fmt;
use crate::terms::grp_notional_principal::unitx::GLN::GLN;

#[derive(Debug, Eq, PartialEq)]

pub struct MWH;

impl MWH {
    pub fn new() -> Self {
        MWH
    }

}

impl fmt::Display for MWH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MWH")
    }
}