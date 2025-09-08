use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct N;

impl N {
    pub fn new() -> Self {
        return N;
    }
    pub fn type_str(&self) -> String {
        return "N Scaling Effect".to_string();
    }
}

impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "N")
    }
}
