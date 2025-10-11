use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct COL;

impl COL {
    pub fn new() -> Self {
        return COL;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl fmt::Display for COL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "COL")
    }
}