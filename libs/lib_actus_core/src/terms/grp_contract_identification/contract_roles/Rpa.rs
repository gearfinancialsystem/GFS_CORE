use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct RPA;

impl RPA {
    pub fn new() -> Self {
        return RPA;
    }
    pub fn type_str(&self) -> String {
        return "RPA contract cont_role".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

