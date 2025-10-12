use std::fmt;


#[derive(Clone, Debug, Eq, PartialEq)]

pub struct IOO;

impl IOO {
    pub fn new() -> Self {
        return IOO;
    }
}


impl fmt::Display for IOO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IOO")
    }
}