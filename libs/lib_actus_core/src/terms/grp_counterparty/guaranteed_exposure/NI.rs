use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct NI;

impl NI {
    pub fn new() -> Self {
        return NI;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for NI {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "nominalValuePlusInterest"
    }
    fn get_name(&self) -> &str {
        "Nominal Value plus Interest"
    }
    fn get_acronym(&self) -> &str {
        "NI"
    }
    fn get_description(&self) -> &str {
        "Nominal value of the exposure plus interest accrued is covered."
    }
}    