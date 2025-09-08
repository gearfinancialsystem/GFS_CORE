use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct INC;

impl INC {
    pub fn new() -> Self {
        INC
    }

}

impl fmt::Display for INC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "INC")
    }
}