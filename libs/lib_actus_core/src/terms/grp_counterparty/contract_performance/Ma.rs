use std::fmt;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct MA;

impl MA {
    pub fn new() -> Self {
        return MA;
    }

}

impl fmt::Display for MA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "MA")


    }
}