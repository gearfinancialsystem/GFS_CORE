use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct MV;

impl MV {
    pub fn new() -> Self {
        return MV;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for MV {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "marketValue"
    }
    fn get_name(&self) -> &str {
        "Market Value"
    }
    fn get_acronym(&self) -> &str {
        "MV"
    }
    fn get_description(&self) -> &str {
        "Market value of the exposure is covered."
    }
}