use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct TRO;

impl TRO {
    pub fn new() -> Self {
        return TRO;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}


