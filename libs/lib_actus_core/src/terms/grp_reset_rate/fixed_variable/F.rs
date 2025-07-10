use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct F;

impl F {
    pub fn new() -> Self {
        return F;
    }
}

impl fmt::Display for F {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F")
    }
}
