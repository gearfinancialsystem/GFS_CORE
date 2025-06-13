use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct UDLP;

impl UDLP {
    pub fn new() -> Self {
        return UDLP;
    }
    pub fn type_str(&self) -> String {
        return "UDLP contract cont_type".to_string();
    }
    pub fn role_sign(&self) -> f64 {
        return 1.0
    }
}

