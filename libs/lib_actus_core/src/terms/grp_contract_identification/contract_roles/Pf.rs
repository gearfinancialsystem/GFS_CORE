use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

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

