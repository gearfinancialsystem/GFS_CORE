use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct SWAPS;
impl SWAPS {
    pub fn new() -> Self {
        return SWAPS;
    }
    pub fn type_str(&self) -> String {
        return "SWAPS contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for SWAPS {
    fn get_option_rank(&self) -> &str {
        "10"
    }
    fn get_identifier(&self) -> &str {
        "swap"
    }
    fn get_name(&self) -> &str {
        "Swap"
    }
    fn get_acronym(&self) -> &str {
        "SWAPS"
    }
    fn get_description(&self) -> &str {
        "An agreement of swapping two legs such as fixed against variable or currency 1 against currency 2 etc."
    }
}