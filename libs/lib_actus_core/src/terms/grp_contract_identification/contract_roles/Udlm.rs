use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct UDLM;

impl UDLM {
    pub fn new() -> Self {
        return UDLM;
    }
    pub fn type_str(&self) -> String {
        return "UDLM contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

impl TraitEnumOptionDescription for UDLM {
    fn get_option_rank(&self) -> &str {
        "12"
    }
    fn get_identifier(&self) -> &str {
        "underlyingMinus"
    }
    fn get_name(&self) -> &str {
        "Underlying Minus"
    }
    fn get_acronym(&self) -> &str {
        "UDLM"
    }
    fn get_description(&self) -> &str {
        "Contract represents the underlying to a composed contract. Role of the underlying is derived from the parent. When considered a standalone contract the underlying’s creator takes the liability side."
    }
}
