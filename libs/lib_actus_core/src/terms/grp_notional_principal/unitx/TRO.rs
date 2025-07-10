use std::fmt;

#[derive(Debug, Eq, PartialEq)]

pub struct TRO;

impl TRO {
    pub fn new() -> Self {
        return TRO;
    }

}


impl fmt::Display for TRO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TRO")
    }
}