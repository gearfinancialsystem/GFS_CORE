use std::fmt;
use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::E::E;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct B;

impl B {
    pub fn new() -> Self {
        return B;
    }

}

impl fmt::Display for B {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "B")

    }
}