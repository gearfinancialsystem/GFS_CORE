use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct UDLP;

impl UDLP {
    pub fn new() -> Self {
        return UDLP;
    }
    pub fn type_str(&self) -> String {
        return "UDLP contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for UDLP {
    fn get_option_rank(&self) -> &str {
        "11"
    }
    fn get_identifier(&self) -> &str {
        "underlyingPlus"
    }
    fn get_name(&self) -> &str {
        "Underlying Plus"
    }
    fn get_acronym(&self) -> &str {
        "UDLP"
    }
    fn get_description(&self) -> &str {
        "Contract represents the underlying to a composed contract. Role of the underlying is derived from the parent. When considered a standalone contract the underlying’s creator takes the asset side."
    }
}    