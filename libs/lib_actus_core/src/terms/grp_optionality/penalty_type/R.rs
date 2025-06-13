use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct R;

impl R {
    pub fn new() -> Self {
        return R;
    }
    pub fn type_str(&self) -> String {
        return "R Scaling Effect".to_string();
    }
}

