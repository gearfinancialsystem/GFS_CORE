use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct TRO;

impl TRO {
    pub fn new() -> Self {
        return TRO;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}


impl TraitEnumOptionDescription for TRO {
    fn get_option_rank(&self) -> &str {
        "8"
    }
    fn get_identifier(&self) -> &str {
        "troyOunce"
    }
    fn get_name(&self) -> &str {
        "Troy Ounce"
    }
    fn get_acronym(&self) -> &str {
        "TRO"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Troy Ounces."
    }
}    