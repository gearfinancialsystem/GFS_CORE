use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct FXOUT;

impl FXOUT {
    pub fn new() -> Self {
        return FXOUT;
    }
    pub fn type_str(&self) -> String {
        return "FXOUT contract cont_type".to_string();
    }
}


impl TraitEnumOptionDescription for FXOUT {
    fn get_option_rank(&self) -> &str {
        "12"
    }
    fn get_identifier(&self) -> &str {
        "foreignExchangeOutright"
    }
    fn get_name(&self) -> &str {
        "Foreign Exchange Outright"
    }
    fn get_acronym(&self) -> &str {
        "FXOUT"
    }
    fn get_description(&self) -> &str {
        "An agreement of swapping two cash flows in different currencies at a future point in time."
    }
}    