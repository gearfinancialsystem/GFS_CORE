

#[derive(Debug, Eq, PartialEq)]

pub struct PF;

impl PF {
    pub fn new() -> Self {
        return PF;
    }
    pub fn type_str(&self) -> String {
        return "PF contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

impl TraitEnumOptionDescription for PF {
    fn get_option_rank(&self) -> &str {
        "5"
    }
    fn get_identifier(&self) -> &str {
        "payFix"
    }
    fn get_name(&self) -> &str {
        "Pay Fix"
    }
    fn get_acronym(&self) -> &str {
        "PF"
    }
    fn get_description(&self) -> &str {
        "Contract creator pays the fixed leg."
    }
}    