use std::fmt;
use crate::terms::grp_notional_principal::increase_decrease::DEC::DEC;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct INC;

impl INC {
    pub fn new() -> Self {
        INC
    }

}

impl fmt::Display for INC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INC")
    }
}