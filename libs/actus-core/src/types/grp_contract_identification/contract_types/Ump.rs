

#[derive(Debug, Eq, PartialEq)]
pub struct UMP;
impl UMP {
    pub fn new() -> Self {
        return UMP;
    }
    pub fn type_str(&self) -> String {
        return "UMP contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for UMP {
    fn get_option_rank(&self) -> &str {
        "6"
    }
    fn get_identifier(&self) -> &str {
        "undefinedMaturityProfile"
    }
    fn get_name(&self) -> &str {
        "Undefined Maturity Profile"
    }
    fn get_acronym(&self) -> &str {
        "UMP"
    }
    fn get_description(&self) -> &str {
        "Interest paying cash accounts (current / savings / etc.)."
    }
}