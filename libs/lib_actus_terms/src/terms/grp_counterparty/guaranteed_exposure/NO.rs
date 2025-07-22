use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct NO;

impl NO {
    pub fn new() -> Self {
        return NO;
    }
}

impl fmt::Display for NO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NO")
    }
}