use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct P;

impl P {
    pub fn new() -> Self {
        return P;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for P {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "put"
    }
    fn get_name(&self) -> &str {
        "Put"
    }
    fn get_acronym(&self) -> &str {
        "P"
    }
    fn get_description(&self) -> &str {
        "Put option."
    }
}    