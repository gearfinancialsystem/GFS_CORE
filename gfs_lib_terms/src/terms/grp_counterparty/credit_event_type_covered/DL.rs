use std::fmt;


#[derive(Debug, Eq, PartialEq)]

pub struct DL;

impl DL {
    pub fn new() -> Self {
        DL
    }

}

impl fmt::Display for DL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DL")


    }
}