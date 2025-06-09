use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct LAM;

impl LAM {
    pub fn new() -> Self {
        return LAM;
    }
    pub fn type_str(&self) -> String {
        return "LAM contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for LAM {
    fn get_option_rank(&self) -> &str {
        "3"
    }
    fn get_identifier(&self) -> &str {
        "linearAmortizer"
    }
    fn get_name(&self) -> &str {
        "Linear Amortizer"
    }
    fn get_acronym(&self) -> &str {
        "LAM"
    }
    fn get_description(&self) -> &str {
        "Lending agreements with fixed principal repayment amounts and variable interest payments."
    }
}