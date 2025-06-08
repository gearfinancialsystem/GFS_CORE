use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct S;
impl S {
    pub fn new() -> Self {
        S
    }
    pub fn type_str(&self) -> String {
        return "S".to_string();
    }
}

impl TraitEnumOptionDescription for S {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "cashSettlement"
    }
    fn get_name(&self) -> &str {
        "Cash Settlement"
    }
    fn get_acronym(&self) -> &str {
        "S"
    }
    fn get_description(&self) -> &str {
        "The market value of the underlying is settled."
    }
}