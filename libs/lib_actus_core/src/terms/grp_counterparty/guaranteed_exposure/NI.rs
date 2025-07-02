use std::fmt;
use crate::terms::grp_counterparty::guaranteed_exposure::MV::MV;

#[derive(Clone, 
    Debug, Eq, PartialEq)]

pub struct NI;

impl NI {
    pub fn new() -> Self {
        return NI;
    }

}

impl fmt::Display for NI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NI")
    }
}