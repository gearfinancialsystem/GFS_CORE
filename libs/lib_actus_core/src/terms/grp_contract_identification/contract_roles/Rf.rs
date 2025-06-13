use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct RF;

impl RF {
    pub fn new() -> Self {
        return RF;
    }
    pub fn type_str(&self) -> String {
        return "RF contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

