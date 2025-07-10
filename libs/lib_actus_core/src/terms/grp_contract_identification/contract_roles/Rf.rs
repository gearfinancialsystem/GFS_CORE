use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct RF;

impl RF {
    pub fn new() -> Self {
        return RF;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl fmt::Display for RF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RF")
    }
}