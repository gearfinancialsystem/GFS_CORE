use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct SEL;

impl SEL {
    pub fn new() -> Self {
        return SEL;
    }
    pub fn type_str(&self) -> String {
        return "SEL contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

impl TraitEnumOptionDescription for SEL {
    fn get_option_rank(&self) -> &str {
        "7"
    }
    fn get_identifier(&self) -> &str {
        "seller"
    }
    fn get_name(&self) -> &str {
        "Seller"
    }
    fn get_acronym(&self) -> &str {
        "SEL"
    }
    fn get_description(&self) -> &str {
        "Contract creator holds the obligation to sell the underlying / deliver the option."            }
}