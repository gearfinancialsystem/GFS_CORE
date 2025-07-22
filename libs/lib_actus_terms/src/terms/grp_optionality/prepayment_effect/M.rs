use std::fmt;

#[derive(Debug, Eq, PartialEq)]

pub struct M;

impl M {
    pub fn new() -> Self {
        return M;
    }
}
impl fmt::Display for M {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "M")
    }
}
