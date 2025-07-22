use std::fmt;


#[derive(Clone, Debug, Eq, PartialEq)]

pub struct V;

impl V {
    pub fn new() -> Self {
        V
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V")
    }
}
