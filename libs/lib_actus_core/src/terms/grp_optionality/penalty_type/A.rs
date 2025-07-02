use std::fmt;
use crate::terms::grp_optionality::prepayment_effect::M::M;

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