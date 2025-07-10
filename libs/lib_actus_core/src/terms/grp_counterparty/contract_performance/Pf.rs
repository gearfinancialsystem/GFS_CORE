use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct PF;

impl PF {
    pub fn new() -> Self {
        return PF;
    }

}

impl fmt::Display for PF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "PF")


    }
}