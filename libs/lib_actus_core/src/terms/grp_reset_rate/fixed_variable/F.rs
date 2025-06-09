use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct F;

impl F {
    pub fn new() -> Self {
        return F;
    }
    pub fn type_str(&self) -> String {
        return "Fixed".to_string();
    }
}

impl TraitEnumOptionDescription for F {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "fixedRate"
    }
    fn get_name(&self) -> &str {
        "Fixed Rate"
    }
    fn get_acronym(&self) -> &str {
        "F"
    }
    fn get_description(&self) -> &str {
        "Rate spread represents a fixed rate."
    }
}