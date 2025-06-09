use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct TON;

impl TON {
    pub fn new() -> Self {
        return TON;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for TON {
    fn get_option_rank(&self) -> &str {
        "7"
    }
    fn get_identifier(&self) -> &str {
        "tons"
    }
    fn get_name(&self) -> &str {
        "Tons"
    }
    fn get_acronym(&self) -> &str {
        "TON"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Tons."
    }
}    