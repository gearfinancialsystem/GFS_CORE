use std::fmt;
use crate::terms::grp_optionality::option_type::C::C;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct CP;

impl CP {
    pub fn new() -> Self {
        return CP;
    }

}

impl fmt::Display for CP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CP")
    }
}