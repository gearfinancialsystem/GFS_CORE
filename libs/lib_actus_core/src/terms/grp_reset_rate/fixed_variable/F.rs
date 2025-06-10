use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct F;

impl F {
    pub fn new() -> Self {
        return F;
    }
    pub fn type_str(&self) -> String {
        return "Fixed".to_string();
    }
}

