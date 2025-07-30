use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]

pub struct J;

impl J {
    pub fn new() -> Self {
        J
    }
    
}

impl fmt::Display for J {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "J")
    }
}