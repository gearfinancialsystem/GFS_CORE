use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct DECR;

impl DECR {
    pub fn new() -> Self {
        return DECR;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for DECR {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "fromAbove"
    }
    fn get_name(&self) -> &str {
        "From Above"
    }
    fn get_acronym(&self) -> &str {
        "DECR"
    }
    fn get_description(&self) -> &str {
        "Boundary action if observed market object value less than or equal to boundary value at a monitor time."
    }
}    