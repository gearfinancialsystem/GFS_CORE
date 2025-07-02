use std::fmt;
use crate::terms::grp_notional_principal::unitx::MWH::MWH;

#[derive(Debug, Eq, PartialEq)]

pub struct PND;

impl PND {
    pub fn new() -> Self {
        return PND;
    }

}
impl fmt::Display for PND {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PND")
    }
}