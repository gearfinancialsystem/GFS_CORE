use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct BRL;
impl BRL {
    pub fn new() -> Self {
        return BRL;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for BRL {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "barrel"
    }
    fn get_name(&self) -> &str {
        "Barrel"
    }
    fn get_acronym(&self) -> &str {
        "BRL"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Barrels."
    }
}