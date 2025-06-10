use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct FXOUT;

impl FXOUT {
    pub fn new() -> Self {
        return FXOUT;
    }
    pub fn type_str(&self) -> String {
        return "FXOUT contract cont_type".to_string();
    }
}


