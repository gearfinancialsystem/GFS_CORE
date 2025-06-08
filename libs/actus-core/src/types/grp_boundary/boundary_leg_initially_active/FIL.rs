use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct FIL;

impl FIL {
    pub fn new() -> Self {
        return FIL;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for FIL {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "firstLeg"
    }
    fn get_name(&self) -> &str {
        "First Leg"
    }
    fn get_acronym(&self) -> &str {
        "FIL"
    }
    fn get_description(&self) -> &str {
        "the first leg is active when the boundary controlled switch contract is initialized."
    }
}    