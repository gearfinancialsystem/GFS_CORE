use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct OPTNS;

impl OPTNS {
    pub fn new() -> Self {
        return OPTNS;
    }
    pub fn type_str(&self) -> String {
        return "OPTNS contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for OPTNS {
    fn get_option_rank(&self) -> &str {
        "15"
    }
    fn get_identifier(&self) -> &str {
        "option"
    }
    fn get_name(&self) -> &str {
        "Option"
    }
    fn get_acronym(&self) -> &str {
        "OPTNS"
    }
    fn get_description(&self) -> &str {
        "Different terms of options on buying an underlying instrument at a fixed price in the future."
    }
}