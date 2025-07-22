use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct CNO;

impl CNO {
    pub fn new() -> Self {
        return CNO;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl fmt::Display for CNO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CNO")
    }
}