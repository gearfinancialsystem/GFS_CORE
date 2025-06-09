use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct Y;

impl Y {
    pub fn new() -> Self {
        return Y;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for Y {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "isClearingHouse"
    }
    fn get_name(&self) -> &str {
        "Is Clearing House"
    }
    fn get_acronym(&self) -> &str {
        "Y"
    }
    fn get_description(&self) -> &str {
        "Contract creator is the clearing house."
    }
}