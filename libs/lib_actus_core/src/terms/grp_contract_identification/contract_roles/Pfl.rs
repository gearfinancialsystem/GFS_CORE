use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

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

