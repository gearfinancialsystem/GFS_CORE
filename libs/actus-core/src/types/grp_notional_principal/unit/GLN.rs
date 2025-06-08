use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct GLN;

impl GLN {
    pub fn new() -> Self {
        return GLN;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
impl TraitEnumOptionDescription for GLN {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "gallon"
    }
    fn get_name(&self) -> &str {
        "Gallon"
    }
    fn get_acronym(&self) -> &str {
        "GLN"
    }
    fn get_description(&self) -> &str {
        "Physical unit of the contract is Gallons."
    }
}