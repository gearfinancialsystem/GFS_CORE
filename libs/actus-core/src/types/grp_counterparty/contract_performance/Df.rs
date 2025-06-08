

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DF;


impl DF {
    pub fn new() -> Self {
        return DF;
    }
    pub fn type_str(&self) -> String {
        return "DF contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for DF {
    fn get_option_rank(&self) -> &str {
        "3"
    }
    fn get_identifier(&self) -> &str {
        "default"
    }
    fn get_name(&self) -> &str {
        "Default"
    }
    fn get_acronym(&self) -> &str {
        "DF"
    }
    fn get_description(&self) -> &str {
        "Contract defaulted on payment obligations according to Delinquency Period."
    }
}