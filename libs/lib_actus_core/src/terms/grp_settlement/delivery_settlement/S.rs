use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct S;

impl S {
    pub fn new() -> Self {
        S
    }
    pub fn type_str(&self) -> String {
        return "S".to_string();
    }
}
