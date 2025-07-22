use std::fmt;

#[derive(Debug, Eq, PartialEq)]

pub struct DF;

impl DF {
    pub fn new() -> Self {
         DF
    }

}

impl fmt::Display for DF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DF")


    }
}