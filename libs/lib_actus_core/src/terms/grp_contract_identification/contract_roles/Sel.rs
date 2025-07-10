use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct SEL;

impl SEL {
    pub fn new() -> Self {
        return SEL;
    }

    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

impl fmt::Display for SEL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SEL")
    }
}