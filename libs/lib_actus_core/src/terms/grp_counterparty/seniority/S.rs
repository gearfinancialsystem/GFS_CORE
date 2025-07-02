use std::fmt;
use crate::terms::grp_counterparty::seniority::J::J;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]

pub struct S;

impl S {
    pub fn new() -> Self {
        S
    }
}
impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S")
    }
}
