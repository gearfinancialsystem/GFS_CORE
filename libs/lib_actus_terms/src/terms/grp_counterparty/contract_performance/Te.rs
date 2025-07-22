use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct TE;

impl TE {
    pub fn new() -> Self {
        return TE;
    }

}


impl fmt::Display for TE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TE")
    }
}