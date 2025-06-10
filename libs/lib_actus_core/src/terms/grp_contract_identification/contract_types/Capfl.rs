use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct CAPFL;

impl CAPFL {
    pub fn new() -> Self {
        return CAPFL;
    }
    pub fn type_str(&self) -> String {
        return "CAPFL contract cont_type".to_string();
    }
}


