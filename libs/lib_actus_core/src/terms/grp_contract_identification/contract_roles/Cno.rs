use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct CNO;

impl CNO {
    pub fn new() -> Self {
        return CNO;
    }
    pub fn type_str(&self) -> String {
        return "CNO contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

