
#[derive(Debug, Eq, PartialEq)]

pub struct RF;

impl RF {
    pub fn new() -> Self {
        return RF;
    }
    pub fn type_str(&self) -> String {
        return "RF contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for RF {
    fn get_option_rank(&self) -> &str {
        "4"
    }
    fn get_identifier(&self) -> &str {
        "receiveFix"
    }
    fn get_name(&self) -> &str {
        "Receive Fix"
    }
    fn get_acronym(&self) -> &str {
        "RF"
    }
    fn get_description(&self) -> &str {
        "Contract creator receives the fixed leg."
    }
}