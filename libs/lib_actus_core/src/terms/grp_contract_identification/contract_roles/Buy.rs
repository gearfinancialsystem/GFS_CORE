use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct BUY;

impl BUY {
    pub fn new() -> Self {
        return BUY;
    }
    pub fn type_str(&self) -> String {
        return "BUY contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for BUY {
    fn get_option_rank(&self) -> &str {
        "6"
    }
    fn get_identifier(&self) -> &str {
        "buyer"
    }
    fn get_name(&self) -> &str {
        "Buyer"
    }
    fn get_acronym(&self) -> &str {
        "BUY"
    }
    fn get_description(&self) -> &str {
        "Contract creator holds the right to buy the underlying / exercise the option."
    }
}    