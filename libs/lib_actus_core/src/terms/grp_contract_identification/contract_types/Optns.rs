use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct OPTNS;

impl OPTNS {
    pub fn new() -> Self {
        return OPTNS;
    }
    pub fn type_str(&self) -> String {
        return "OPTNS contract cont_type".to_string();
    }
}

