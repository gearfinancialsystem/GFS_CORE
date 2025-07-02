use std::fmt;
use crate::terms::grp_contract_identification::contract_roles::Pf::PF;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct PFL;

impl PFL {
    pub fn new() -> Self {
        return PFL;
    }

    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

impl fmt::Display for PFL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PFL")
    }
}