use std::fmt;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct TE;

impl TE {
    pub fn new() -> Self {
        return TE;
    }

}


impl fmt::Display for TE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TE")
    }
}