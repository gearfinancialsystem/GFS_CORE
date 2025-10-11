use std::fmt;

#[derive(Debug, Eq, PartialEq)]

pub struct Y;

impl Y {
    pub fn new() -> Self {
        Y
    }
}

impl fmt::Display for Y {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Y")
    }
}