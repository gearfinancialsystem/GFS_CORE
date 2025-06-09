use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct MA;

impl MA {
    pub fn new() -> Self {
        return MA;
    }
    pub fn type_str(&self) -> String {
        return "MA contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for MA {
    fn get_option_rank(&self) -> &str {
        "4"
    }
    fn get_identifier(&self) -> &str {
        "matured"
    }
    fn get_name(&self) -> &str {
        "Matured"
    }
    fn get_acronym(&self) -> &str {
        "MA"
    }
    fn get_description(&self) -> &str {
        "Contract matured."
    }
}