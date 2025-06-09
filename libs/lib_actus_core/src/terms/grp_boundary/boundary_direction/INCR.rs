use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct INCR;

impl INCR {
    pub fn new() -> Self {
        return INCR;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for INCR {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "fromBelow"
    }
    fn get_name(&self) -> &str {
        "From Below"
    }
    fn get_acronym(&self) -> &str {
        "INCR"
    }
    fn get_description(&self) -> &str {
        "Boundary effect is trigerred if the observed underlying asset value is greater than or equal to the boundary value at a monitor time."
    }
}