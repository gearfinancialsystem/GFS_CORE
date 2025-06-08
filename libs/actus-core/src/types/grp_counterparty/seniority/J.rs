use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct J;

impl J {
    pub fn new() -> Self {
        return J;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for J {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "junior"
    }
    fn get_name(&self) -> &str {
        "Junior"
    }
    fn get_acronym(&self) -> &str {
        "J"
    }
    fn get_description(&self) -> &str {
        "Contract represents junior debt."
    }
}