

#[derive(Debug, Eq, PartialEq)]
pub struct SWPPV;

impl SWPPV {
    pub fn new() -> Self {
        return SWPPV;
    }
    pub fn type_str(&self) -> String {
        return "SWPPV contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for SWPPV {
    fn get_option_rank(&self) -> &str {
        "11"
    }
    fn get_identifier(&self) -> &str {
        "plainVanillaSwap"
    }
    fn get_name(&self) -> &str {
        "Plain Vanilla Swap"
    }
    fn get_acronym(&self) -> &str {
        "SWPPV"
    }
    fn get_description(&self) -> &str {
        "Plain vanilla interest rate swaps."
    }
}    