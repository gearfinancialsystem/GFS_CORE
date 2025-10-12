use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct ONO;

impl ONO {
    pub fn new() -> Self {
        return ONO;
    }

}

impl fmt::Display for ONO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ONO")
    }
}