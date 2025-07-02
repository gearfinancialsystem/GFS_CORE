use std::fmt;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct PF;

impl PF {
    pub fn new() -> Self {
        return PF;
    }

}

impl fmt::Display for PF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "PF")


    }
}