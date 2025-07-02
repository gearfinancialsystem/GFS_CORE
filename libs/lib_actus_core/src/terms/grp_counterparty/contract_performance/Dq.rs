use std::fmt;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct DQ;

impl DQ {
    pub fn new() -> Self {
        return DQ;
    }

}

impl fmt::Display for DQ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DQ")


    }
}