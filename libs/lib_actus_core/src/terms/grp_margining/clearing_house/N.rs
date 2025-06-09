use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct N;

impl N {
    pub fn new() -> Self {
        return N;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for N {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "isNotClearingHouse"
    }
    fn get_name(&self) -> &str {
        "Is Not Clearing House"
    }
    fn get_acronym(&self) -> &str {
        "N"
    }
    fn get_description(&self) -> &str {
        "Contract creator is not the clearing house."
    }
}