

#[derive(Debug, Eq, PartialEq)]
pub struct FUTUR;

impl FUTUR {
    pub fn new() -> Self {
        return FUTUR;
    }
    pub fn type_str(&self) -> String {
        return "FUTUR contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for FUTUR {
    fn get_option_rank(&self) -> &str {
        "14"
    }
    fn get_identifier(&self) -> &str {
        "future"
    }
    fn get_name(&self) -> &str {
        "Future"
    }
    fn get_acronym(&self) -> &str {
        "FUTUR"
    }
    fn get_description(&self) -> &str {
        "An agreement of exchanging an underlying instrument against a fixed price in the future."
    }
}    