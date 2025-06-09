use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct PF;

impl PF {
    pub fn new() -> Self {
        return PF;
    }
    pub fn type_str(&self) -> String {
        return "PF contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for PF {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "performant"
    }
    fn get_name(&self) -> &str {
        "Performant"
    }
    fn get_acronym(&self) -> &str {
        "PF"
    }
    fn get_description(&self) -> &str {
        "Contract is performing according to terms and conditions."
    }
}