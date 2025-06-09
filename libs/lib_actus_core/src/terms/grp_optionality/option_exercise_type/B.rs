use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct B;

impl B {
    pub fn new() -> Self {
        return B;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for B {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "bermudan"
    }
    fn get_name(&self) -> &str {
        "Bermudan"
    }
    fn get_acronym(&self) -> &str {
        "B"
    }
    fn get_description(&self) -> &str {
        "Bermudan-cont_type exercise."
    }
}