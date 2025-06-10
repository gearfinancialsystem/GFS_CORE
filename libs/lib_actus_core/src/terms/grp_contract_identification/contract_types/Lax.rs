use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct LAX;

impl LAX {
    pub fn new() -> Self {
        return LAX;
    }
    pub fn type_str(&self) -> String {
        return "LAX contract cont_type".to_string();
    }
}

