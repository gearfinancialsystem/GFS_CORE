use std::fmt;
use crate::terms::grp_notional_principal::Unit::Unit;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct INO;

impl INO {
    pub fn new() -> Self {
        INO
    }

}


impl fmt::Display for INO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INO")
    }
}