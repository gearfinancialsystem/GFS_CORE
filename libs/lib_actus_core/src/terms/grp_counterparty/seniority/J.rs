use std::fmt;
use crate::terms::grp_counterparty::guaranteed_exposure::MV::MV;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]

pub struct J;

impl J {
    pub fn new() -> Self {
        J
    }
    
}

impl fmt::Display for J {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "J")
    }
}