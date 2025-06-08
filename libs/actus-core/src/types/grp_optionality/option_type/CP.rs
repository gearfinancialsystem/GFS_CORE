use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct CP;

impl CP {
    pub fn new() -> Self {
        return CP;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for CP {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "callPut"
    }
    fn get_name(&self) -> &str {
        "Call-Put"
    }
    fn get_acronym(&self) -> &str {
        "CP"
    }
    fn get_description(&self) -> &str {
        "Combination of call and put option."
    }
}