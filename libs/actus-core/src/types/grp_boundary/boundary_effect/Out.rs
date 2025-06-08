use crate::terms::grp_boundary::boundary_effect::Insel::INSEL;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, Clone, Copy, PartialEq)]

pub struct OUT;

impl OUT {
    pub fn new() -> Self {
        return OUT;
    }
    pub fn type_str(&self) -> String {
        return "OUT contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for OUT {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "knockOUTCurrentLeg"
    }
    fn get_name(&self) -> &str {
        "KnockOUT Current Leg"
    }
    fn get_acronym(&self) -> &str {
        "OUT"
    }
    fn get_description(&self) -> &str {
        "effect of boundary crossing is to knockOUT any active contract so there is no active contract after the boundary crossing; monitoring of the boundary stops."
    }
}