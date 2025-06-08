use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct PND;

impl PND {
    pub fn new() -> Self {
        return PND;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for PND {
    fn get_option_rank(&self) -> &str {
        "5"
    }
    fn get_identifier(&self) -> &str {
        "pounds"
    }
    fn get_name(&self) -> &str {
        "Pounds"
    }
    fn get_acronym(&self) -> &str {
        "PND"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Pounds."
    }
}    