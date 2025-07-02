use std::fmt;
use crate::terms::grp_fees::fee_basis::N::N;
use crate::terms::grp_fees::FeeBasis::FeeBasis;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct A;

impl A {
    pub fn new() -> Self {
        return A;
    }

}

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 
        write!(f, "A")

    }
}