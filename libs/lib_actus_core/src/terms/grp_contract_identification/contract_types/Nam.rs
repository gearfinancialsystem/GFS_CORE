use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct NAM;

impl NAM {
    pub fn new() -> Self {
        return NAM;
    }
    pub fn type_str(&self) -> String {
        return "NAM contract cont_type".to_string();
    }
}

