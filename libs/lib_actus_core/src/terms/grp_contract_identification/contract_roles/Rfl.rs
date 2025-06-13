use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

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


