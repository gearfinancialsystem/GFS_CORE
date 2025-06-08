use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct INC;

impl INC {
    pub fn new() -> Self {
        INC
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for INC {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "increase"
    }
    fn get_name(&self) -> &str {
        "Increase"
    }
    fn get_acronym(&self) -> &str {
        "INC"
    }
    fn get_description(&self) -> &str {
        "Notional is increased in this period."
    }
} 