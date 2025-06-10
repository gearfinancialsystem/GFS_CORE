use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct UMP;
impl UMP {
    pub fn new() -> Self {
        return UMP;
    }
    pub fn type_str(&self) -> String {
        return "UMP contract cont_type".to_string();
    }
}

