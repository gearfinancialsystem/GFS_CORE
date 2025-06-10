use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct SWPPV;

impl SWPPV {
    pub fn new() -> Self {
        return SWPPV;
    }
    pub fn type_str(&self) -> String {
        return "SWPPV contract cont_type".to_string();
    }
}

