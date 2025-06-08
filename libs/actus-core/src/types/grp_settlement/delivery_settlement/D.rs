use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct D;
impl D {
    pub fn new() -> Self {
        D
    }
    pub fn type_str(&self) -> String {
        return "D".to_string();
    }
}

impl TraitEnumOptionDescription for D {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "physicalSettlement"
    }
    fn get_name(&self) -> &str {
        "Physical Settlement"
    }
    fn get_acronym(&self) -> &str {
        "D"
    }
    fn get_description(&self) -> &str {
        "The underlying is delivered physically."
    }
}