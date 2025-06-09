use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct TE;

impl TE {
    pub fn new() -> Self {
        return TE;
    }
    pub fn type_str(&self) -> String {
        return "TE contract cont_type".to_string();
    }
}


impl TraitEnumOptionDescription for TE {
    fn get_option_rank(&self) -> &str {
        "5"
    }
    fn get_identifier(&self) -> &str {
        "terminated"
    }
    fn get_name(&self) -> &str {
        "Terminated"
    }
    fn get_acronym(&self) -> &str {
        "TE"
    }
    fn get_description(&self) -> &str {
        "Contract has been terminated."
    }
}    