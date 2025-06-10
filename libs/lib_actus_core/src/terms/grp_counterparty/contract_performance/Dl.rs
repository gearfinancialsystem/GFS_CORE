use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct DL;

impl DL {
    pub fn new() -> Self {
        return DL;
    }
    pub fn type_str(&self) -> String {
        return "DL contract cont_type".to_string();
    }
}

