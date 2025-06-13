use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct OOO;

impl OOO {
    pub fn new() -> Self {
        return OOO;
    }
    pub fn type_str(&self) -> String {
        return "OOO Scaling Effect".to_string();
    }
}

