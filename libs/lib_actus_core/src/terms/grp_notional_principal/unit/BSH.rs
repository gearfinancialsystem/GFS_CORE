use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct BSH;

impl BSH {
    pub fn new() -> Self {
        return BSH;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
impl TraitEnumOptionDescription for BSH {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "bushel"
    }
    fn get_name(&self) -> &str {
        "Bushel"
    }
    fn get_acronym(&self) -> &str {
        "BSH"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Bushel."
    }
}  