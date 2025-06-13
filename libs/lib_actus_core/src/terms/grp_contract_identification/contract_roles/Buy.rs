use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct BUY;

impl BUY {
    pub fn new() -> Self {
        return BUY;
    }
    pub fn type_str(&self) -> String {
        return "BUY contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

