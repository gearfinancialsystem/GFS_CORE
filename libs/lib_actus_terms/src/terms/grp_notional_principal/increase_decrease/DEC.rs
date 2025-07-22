use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct DEC;

impl DEC {
    pub fn new() -> Self {
        DEC
    }

}

impl fmt::Display for DEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEC")
    }
}