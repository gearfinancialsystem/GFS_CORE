use std::fmt;

#[derive(Debug, Eq, PartialEq)]

pub struct DQ;

impl DQ {
    pub fn new() -> Self {
         DQ
    }

}
impl fmt::Display for DQ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DQ")


    }
}