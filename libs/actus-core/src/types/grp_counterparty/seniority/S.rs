use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct S;

impl S {
    pub fn new() -> Self {
        return S;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for S {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "senior"
    }
    fn get_name(&self) -> &str {
        "Senior"
    }
    fn get_acronym(&self) -> &str {
        "S"
    }
    fn get_description(&self) -> &str {
        "Contract represents senior debt."
    }
}