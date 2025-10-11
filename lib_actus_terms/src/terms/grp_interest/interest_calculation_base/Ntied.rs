use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NTIED;

impl NTIED {
    pub fn new() -> Self {
        NTIED
    }
}

impl fmt::Display for NTIED {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NTIED")
    }
}