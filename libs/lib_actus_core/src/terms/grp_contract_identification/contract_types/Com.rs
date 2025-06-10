use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct COM;

impl COM {
    pub fn new() -> Self {
        return COM;
    }
    pub fn type_str(&self) -> String {
        return "COM contract cont_type".to_string();
    }
}

