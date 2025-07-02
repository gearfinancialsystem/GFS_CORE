use std::fmt;
use crate::terms::grp_counterparty::contract_performance::Df::DF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct DL;

impl DL {
    pub fn new() -> Self {
        return DL;
    }

}

impl fmt::Display for DL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DL")


    }
}