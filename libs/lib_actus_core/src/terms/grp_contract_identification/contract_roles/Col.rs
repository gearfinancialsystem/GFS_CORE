use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct COL;

impl COL {
    pub fn new() -> Self {
        return COL;
    }
    pub fn type_str(&self) -> String {
        return "COL contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

