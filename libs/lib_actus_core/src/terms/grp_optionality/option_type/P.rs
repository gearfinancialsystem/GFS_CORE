use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct P;

impl P {
    pub fn new() -> Self {
        return P;
    }

}

impl fmt::Display for P {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P")
    }
}