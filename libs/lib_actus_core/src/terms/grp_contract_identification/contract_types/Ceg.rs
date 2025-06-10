use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct CEG;

impl CEG {
    pub fn new() -> Self {
        return CEG;
    }
    pub fn type_str(&self) -> String {
        return "CEG contract cont_type".to_string();
    }
}



        