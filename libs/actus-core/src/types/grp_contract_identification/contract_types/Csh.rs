

#[derive(Debug, Eq, PartialEq)]
pub struct CSH;

impl CSH {
    pub fn new() -> Self {
        return CSH;
    }
    pub fn type_str(&self) -> String {
        return "CSH contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for CSH {
    fn get_option_rank(&self) -> &str {
        "7"
    }
    fn get_identifier(&self) -> &str {
        "cash"
    }
    fn get_name(&self) -> &str {
        "Cash"
    }
    fn get_acronym(&self) -> &str {
        "CSH"
    }
    fn get_description(&self) -> &str {
        "Represents cash holdings."
    }
}   