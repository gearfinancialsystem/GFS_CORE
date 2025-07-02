use std::fmt;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct BUY;

impl BUY {
    pub fn new() -> Self {
        return BUY;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl fmt::Display for BUY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BUY")
    }
}