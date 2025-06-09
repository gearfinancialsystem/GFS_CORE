use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct C;

impl C {
    pub fn new() -> Self {
        return C;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}
impl TraitEnumOptionDescription for C {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "call"
    }
    fn get_name(&self) -> &str {
        "Call"
    }
    fn get_acronym(&self) -> &str {
        "C"
    }
    fn get_description(&self) -> &str {
        "Call option."
    }
}