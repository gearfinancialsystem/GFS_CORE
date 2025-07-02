use std::fmt;
use crate::terms::grp_fees::fee_basis::A::A;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct N;

impl N {
    pub fn new() -> Self {
        return N;
    }

}
impl fmt::Display for N {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "N")

    }
}