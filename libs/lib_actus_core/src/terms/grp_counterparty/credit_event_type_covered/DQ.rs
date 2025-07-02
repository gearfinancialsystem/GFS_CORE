use std::fmt;
use crate::terms::grp_counterparty::credit_event_type_covered::DL::DL;

#[derive(Debug, Eq, PartialEq)]

pub struct DQ;

impl DQ {
    pub fn new() -> Self {
         DQ
    }

}
impl fmt::Display for DQ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DQ")


    }
}