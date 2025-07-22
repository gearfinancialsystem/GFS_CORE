use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NTL;

impl NTL {
    pub fn new() -> Self {
        NTL
    }
}

impl fmt::Display for NTL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NTL")
    }
}