use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct IOO;

impl IOO {
    pub fn new() -> Self {
        return IOO;
    }
    pub fn type_str(&self) -> String {
        return "IOO Scaling Effect".to_string();
    }
}

