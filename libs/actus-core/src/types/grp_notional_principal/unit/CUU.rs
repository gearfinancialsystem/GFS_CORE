use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct CUU;

impl CUU {
    pub fn new() -> Self {
        return CUU;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for CUU {
    fn get_option_rank(&self) -> &str {
        "3"
    }
    fn get_identifier(&self) -> &str {
        "currencyUnit"
    }
    fn get_name(&self) -> &str {
        "Currency Unit"
    }
    fn get_acronym(&self) -> &str {
        "CUU"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Currency Units."
    }
} 