use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Eq, Clone, Copy, PartialEq)]

pub struct INFIL;

impl INFIL {
    pub fn new() -> Self {
        return INFIL;
    }
    pub fn type_str(&self) -> String {
        return "INFIL contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for INFIL {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "knockINFirstLeg"
    }
    fn get_name(&self) -> &str {
        "KnockIN First Leg"
    }
    fn get_acronym(&self) -> &str {
        "INFIL"
    }
    
    fn get_description(&self) -> &str {
        "effect of boundary crossing is to knock IN the first leg making this the active contract; monitoring of the boundary stops.\r"
    }
}
