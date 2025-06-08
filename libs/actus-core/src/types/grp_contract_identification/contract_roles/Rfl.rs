

#[derive(Debug, Eq, PartialEq)]

pub struct RFL;

impl RFL {
    pub fn new() -> Self {
        return RFL;
    }
    pub fn type_str(&self) -> String {
        return "RFL contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for RFL {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "receiveFirstLegl"
    }
    fn get_name(&self) -> &str {
        "Receive First Leg"
    }
    fn get_acronym(&self) -> &str {
        "RFL"
    }
    fn get_description(&self) -> &str {
        "Contract creator receives the first leg."
    }
}
