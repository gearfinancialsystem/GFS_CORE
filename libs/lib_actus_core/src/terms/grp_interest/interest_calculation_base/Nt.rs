use std::fmt;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NT;

impl NT {
    pub fn new() -> Self {
        NT
    }
}

impl fmt::Display for NT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NT")
    }
}