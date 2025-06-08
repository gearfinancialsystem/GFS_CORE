

#[derive(Debug, Eq, PartialEq)]
pub struct CEC;

impl CEC {
    pub fn new() -> Self {
        return CEC;
    }
    pub fn type_str(&self) -> String {
        return "CEC contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for CEC {
    fn get_option_rank(&self) -> &str {
        "17"
    }
    fn get_identifier(&self) -> &str {
        "creditEnhancementCollateral"
    }
    fn get_name(&self) -> &str {
        "Credit Enhancement Collateral"
    }
    fn get_acronym(&self) -> &str {
        "CEC"
    }
    fn get_description(&self) -> &str {
        "A collateral securing the scheduled payment obligations of an underlying instrument"
    }
}   