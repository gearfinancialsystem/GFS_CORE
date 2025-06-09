use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct A;

impl A {
    pub fn new() -> Self {
        return A;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for A {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "fixedPenalty"
    }
    fn get_name(&self) -> &str {
        "Fixed Penalty"
    }
    fn get_acronym(&self) -> &str {
        "A"
    }
    fn get_description(&self) -> &str {
        "A fixed amount applies as penalty."
    }
}