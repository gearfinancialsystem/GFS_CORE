use std::fmt;
use crate::terms::grp_optionality::penalty_type::N::N;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct R;

impl R {
    pub fn new() -> Self {
        return R;
    }

}

impl fmt::Display for R {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R")
    }
}