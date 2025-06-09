use crate::terms::grp_boundary::boundary_effect::Infil::INFIL;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, Clone, Copy, PartialEq)]

pub struct INSEL;

impl INSEL {
    pub fn new() -> Self {
        return INSEL;
    }
    pub fn type_str(&self) -> String {
        return "INSEL contract cont_type".to_string();
    }
}
impl TraitEnumOptionDescription for INSEL {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "knockINSecondLeg"
    }
    fn get_name(&self) -> &str {
        "KnockIN Second Leg"
    }
    fn get_acronym(&self) -> &str {
        "INSEL"
    }
    fn get_description(&self) -> &str {
        "effect of boundary crossing is to knock IN the Second Leg making this the active contract; monitoring of the boundary stops.\r"
    }
}