use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct UDLM;

impl UDLM {
    pub fn new() -> Self {
        return UDLM;
    }
    pub fn type_str(&self) -> String {
        return "UDLM contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

