use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct RPL;

impl RPL {
    pub fn new() -> Self {
        return RPL;
    }
    pub fn type_str(&self) -> String {
        return "RPLs contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return -1.0
    }
}

