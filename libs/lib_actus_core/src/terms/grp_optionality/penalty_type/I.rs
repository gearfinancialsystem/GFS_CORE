use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct I;

impl I {
    pub fn new() -> Self {
        return I;
    }
    pub fn type_str(&self) -> String {
        return "I Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for I {
    fn get_option_rank(&self) -> &str {
        "3"
    }
    fn get_identifier(&self) -> &str {
        "interestRateDifferential"
    }
    fn get_name(&self) -> &str {
        "Interest Rate Differential"
    }
    fn get_acronym(&self) -> &str {
        "I"
    }
    fn get_description(&self) -> &str {
        "A penalty based on the current interest rate differential relative to the notional outstanding applies."
    }
}