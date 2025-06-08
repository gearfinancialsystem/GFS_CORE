

#[derive(Debug, Eq, PartialEq)]

pub struct RPL;

impl RPL {
    pub fn new() -> Self {
        return RPL;
    }
    pub fn type_str(&self) -> String {
        return "RPLs contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

impl TraitEnumOptionDescription for RPL {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "realPositionLiability"
    }
    fn get_name(&self) -> &str {
        "Real Position Liability"
    }
    fn get_acronym(&self) -> &str {
        "RPL"
    }
    fn get_description(&self) -> &str {
        "Contract creator takes the liability or borrower side."
    }
}    