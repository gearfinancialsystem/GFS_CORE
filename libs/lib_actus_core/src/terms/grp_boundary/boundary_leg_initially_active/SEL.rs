use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct SEL;

impl SEL {
    pub fn new() -> Self {
        return SEL;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for SEL {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "secondLeg"
    }
    fn get_name(&self) -> &str {
        "Second Leg"
    }
    fn get_acronym(&self) -> &str {
        "SEL"
    }
    fn get_description(&self) -> &str {
        "the second leg is active when the boundary controlled switch contract starts."
    }
}