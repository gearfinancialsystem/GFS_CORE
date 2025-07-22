use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct MV;

impl MV {
    pub fn new() -> Self {
        MV
    }

}

impl fmt::Display for MV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MV")
    }
}