use std::fmt;
use crate::terms::grp_calendar::Calendar::Calendar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DF;


impl DF {
    pub fn new() -> Self {
        return DF;
    }

}

impl fmt::Display for DF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "DF")


    }
}