use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct DF;

impl DF {
    pub fn new() -> Self {
        return DF;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for DF {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "default"
    }
    fn get_name(&self) -> &str {
        "Default"
    }
    fn get_acronym(&self) -> &str {
        "DF"
    }
    fn get_description(&self) -> &str {
        "Default of the underlying represents a credit event."
    }
}