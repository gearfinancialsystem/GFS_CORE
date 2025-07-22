use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct I;

impl I {
    pub fn new() -> Self {
        return I;
    }
    pub fn type_str(&self) -> String {
        return "I Scaling Effect".to_string();
    }
}


impl fmt::Display for I {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I")
    }
}