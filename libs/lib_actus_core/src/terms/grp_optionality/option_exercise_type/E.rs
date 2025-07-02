use std::fmt;
use crate::terms::grp_optionality::option_exercise_type::B::B;

#[derive(Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self {
        return E;
    }

}

impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E")
    }
}