use std::fmt;


#[derive(Clone, Debug, Eq, PartialEq)]

pub struct B;

impl B {
    pub fn new() -> Self {
        B
    }
}
impl fmt::Display for B {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "B")
    }
}
