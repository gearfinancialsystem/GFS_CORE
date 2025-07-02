use std::fmt;
use crate::terms::grp_contract_identification::contract_roles::Cno::CNO;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct PF;

impl PF {
    pub fn new() -> Self {
        return PF;
    }

    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

impl fmt::Display for PF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PF")
    }
}