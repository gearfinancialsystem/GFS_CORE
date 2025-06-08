use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct NO;

impl NO {
    pub fn new() -> Self {
        return NO;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for NO {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "nominalValue"
    }
    fn get_name(&self) -> &str {
        "Nominal Value"
    }
    fn get_acronym(&self) -> &str {
        "NO"
    }
    fn get_description(&self) -> &str {
        "Nominal value of the exposure is covered."
    }
}    