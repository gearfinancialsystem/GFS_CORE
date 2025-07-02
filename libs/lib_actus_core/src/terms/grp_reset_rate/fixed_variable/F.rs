use std::fmt;
use crate::terms::grp_optionality::penalty_type::A::A;
use crate::terms::grp_optionality::penalty_type::I::I;
use crate::terms::grp_optionality::penalty_type::N::N;
use crate::terms::grp_optionality::penalty_type::R::R;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::fixed_variable::V::V;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct F;

impl F {
    pub fn new() -> Self {
        return F;
    }
}

impl fmt::Display for F {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F")
    }
}
