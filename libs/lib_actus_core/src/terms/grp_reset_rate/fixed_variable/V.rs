use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct V;

impl V {
    pub fn new() -> Self {
        return V;
    }
    pub fn type_str(&self) -> String {
        return "Variable".to_string();
    }
}

impl TraitEnumOptionDescription for V {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "variableRate"
    }
    fn get_name(&self) -> &str {
        "Variable Rate"
    }
    fn get_acronym(&self) -> &str {
        "V"
    }
    fn get_description(&self) -> &str {
        "Rate spread represents the spread on top of a reference rate."
    }
}    