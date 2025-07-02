use std::fmt;
use crate::terms::grp_optionality::penalty_type::A::A;
use crate::terms::grp_optionality::penalty_type::I::I;
use crate::terms::grp_optionality::penalty_type::N::N;
use crate::terms::grp_optionality::penalty_type::R::R;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;

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
