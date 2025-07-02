use std::fmt;
use crate::terms::grp_optionality::option_exercise_type::A::A;

#[derive(Debug, Eq, PartialEq)]

pub struct B;

impl B {
    pub fn new() -> Self {
        return B;
    }

}

impl fmt::Display for B {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "B")
    }
}