

#[derive(Debug, Eq, PartialEq)]
pub struct COM;

impl COM {
    pub fn new() -> Self {
        return COM;
    }
    pub fn type_str(&self) -> String {
        return "COM contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for COM {
    fn get_option_rank(&self) -> &str {
        "9"
    }
    fn get_identifier(&self) -> &str {
        "commodity"
    }
    fn get_name(&self) -> &str {
        "Commodity"
    }
    fn get_acronym(&self) -> &str {
        "COM"
    }
    fn get_description(&self) -> &str {
        "Represents commodities."
    }
}   