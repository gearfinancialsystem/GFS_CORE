use std::fmt;
use crate::terms::grp_contract_identification::contract_roles::Udl::UDL;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct UDLM;

impl UDLM {
    pub fn new() -> Self {
        return UDLM;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl fmt::Display for UDLM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UDLM")
    }
}