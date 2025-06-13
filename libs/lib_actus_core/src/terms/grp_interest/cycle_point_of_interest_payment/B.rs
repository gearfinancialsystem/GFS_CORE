use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct B;

impl B {
    pub fn new() -> Self {
        return B;
    }
    pub fn type_str(&self) -> String {
        return "B Scaling Effect".to_string();
    }
}

