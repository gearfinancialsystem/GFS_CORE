use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct DL;

impl DL {
    pub fn new() -> Self {
        return DL;
    }
    pub fn type_str(&self) -> String {
        return "DL contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for DL {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "delayed"
    }
    fn get_name(&self) -> &str {
        "Delayed"
    }
    fn get_acronym(&self) -> &str {
        "DL"
    }
    fn get_description(&self) -> &str {
        "Contractual payment obligations are delayed according to the Grace Period."
    }
}    