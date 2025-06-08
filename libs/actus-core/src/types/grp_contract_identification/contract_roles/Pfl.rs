

#[derive(Debug, Eq, PartialEq)]

pub struct PFL;

impl PFL {
    pub fn new() -> Self {
        return PFL;
    }
    pub fn type_str(&self) -> String {
        return "PFL contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

impl TraitEnumOptionDescription for PFL {
    fn get_option_rank(&self) -> &str {
        "3"
    }
    fn get_identifier(&self) -> &str {
        "payFirstLeg"
    }
    fn get_name(&self) -> &str {
        "Pay First Leg"
    }
    fn get_acronym(&self) -> &str {
        "PFL"
    }
    fn get_description(&self) -> &str {
        "Contract creator pays the first leg."
    }
}