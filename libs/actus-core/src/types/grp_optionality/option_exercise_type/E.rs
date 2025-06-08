use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self {
        return E;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for E {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "european"
    }
    fn get_name(&self) -> &str {
        "European"
    }
    fn get_acronym(&self) -> &str {
        "E"
    }
    fn get_description(&self) -> &str {
        "European-cont_type exercise."
    }
}
