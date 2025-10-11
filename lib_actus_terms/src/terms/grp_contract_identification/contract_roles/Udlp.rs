use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct UDLP;

impl UDLP {
    pub fn new() -> Self {
        return UDLP;
    }

    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl fmt::Display for UDLP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UDLP")
    }
}