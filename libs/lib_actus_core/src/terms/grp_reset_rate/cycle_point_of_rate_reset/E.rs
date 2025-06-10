use crate::terms::grp_reset_rate::cycle_point_of_rate_reset::B::B;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self {
        return E;
    }
    pub fn type_str(&self) -> String {
        return "E Scaling Effect".to_string();
    }
}

