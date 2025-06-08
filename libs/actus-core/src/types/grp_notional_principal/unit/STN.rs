use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct STN;

impl STN {
    pub fn new() -> Self {
        return STN;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for STN {
    fn get_option_rank(&self) -> &str {
        "6"
    }
    fn get_identifier(&self) -> &str {
        "shortTons"
    }
    fn get_name(&self) -> &str {
        "Short Tons"
    }
    fn get_acronym(&self) -> &str {
        "STN"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Short Tons."
    }
}    