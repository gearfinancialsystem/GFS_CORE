use std::fmt;
use crate::terms::grp_margining::ClearingHouse::ClearingHouse;
use crate::terms::grp_notional_principal::scaling_effect::Ooo::OOO;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct DEC;

impl DEC {
    pub fn new() -> Self {
        DEC
    }

}

impl fmt::Display for DEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEC")
    }
}