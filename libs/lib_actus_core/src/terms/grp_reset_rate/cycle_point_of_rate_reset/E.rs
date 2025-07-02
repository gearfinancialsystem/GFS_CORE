use std::fmt;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self { E }

}

impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "E")

    }
}