use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct CLM;
impl CLM {
    pub fn new() -> Self {
        return CLM;
    }
    pub fn type_str(&self) -> String {
        return "CLM contract cont_type".to_string();
    }
}

