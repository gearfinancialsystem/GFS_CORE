use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct DEC;

impl DEC {
    pub fn new() -> Self {
        DEC
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for DEC {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "decrease"
    }
    fn get_name(&self) -> &str {
        "Decrease"
    }
    fn get_acronym(&self) -> &str {
        "DEC"
    }
    fn get_description(&self) -> &str {
        "Notional is decreased in this period."
    }
} 