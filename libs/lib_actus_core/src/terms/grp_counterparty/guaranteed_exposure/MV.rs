use std::fmt;
use crate::terms::grp_counterparty::credit_event_type_covered::DL::DL;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct MV;

impl MV {
    pub fn new() -> Self {
        MV
    }

}

impl fmt::Display for MV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MV")
    }
}