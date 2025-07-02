use std::fmt;
use crate::terms::grp_margining::clearing_house::Y::Y;
use crate::terms::grp_margining::ClearingHouse::ClearingHouse;

#[derive(Debug, Eq, PartialEq)]

pub struct N;

impl N {
    pub fn new() -> Self {
        N
    }

}

impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "N")
    }
}