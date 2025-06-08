use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct MWH;

impl MWH {
    pub fn new() -> Self {
        return MWH;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for MWH {
    fn get_option_rank(&self) -> &str {
        "4"
    }
    fn get_identifier(&self) -> &str {
        "megaWattHours"
    }
    fn get_name(&self) -> &str {
        "Mega Watt Hours"
    }
    fn get_acronym(&self) -> &str {
        "MWH"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Mega Watt Hours."
    }
}   