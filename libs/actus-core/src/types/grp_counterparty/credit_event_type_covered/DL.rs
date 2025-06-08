use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct DL;

impl DL {
    pub fn new() -> Self {
        return DL;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for DL {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "delayed"
    }
    fn get_name(&self) -> &str {
        "Delayed"
    }
    fn get_acronym(&self) -> &str {
        "DL"
    }
    fn get_description(&self) -> &str {
        "Delay of the underlying represents a credit event."
    }
}    