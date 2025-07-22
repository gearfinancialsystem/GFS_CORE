use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct C;

impl C {
    pub fn new() -> Self {
        return C;
    }
}
impl fmt::Display for C {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "C")
    }
}