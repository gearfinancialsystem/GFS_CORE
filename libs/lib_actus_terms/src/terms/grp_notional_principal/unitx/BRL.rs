use std::fmt;

#[derive(Debug, Eq, PartialEq)]

pub struct BRL;
impl BRL {
    pub fn new() -> Self {
        return BRL;
    }
}

impl fmt::Display for BRL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BRL")
    }
}