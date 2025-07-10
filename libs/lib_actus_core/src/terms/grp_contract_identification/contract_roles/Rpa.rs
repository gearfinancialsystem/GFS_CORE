use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct RPA;

impl RPA {
    pub fn new() -> Self {
        return RPA;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl fmt::Display for RPA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RPA")
    }
}