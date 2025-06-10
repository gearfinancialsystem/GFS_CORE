use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct STK;
impl STK {
    pub fn new() -> Self {
        return STK;
    }
    pub fn type_str(&self) -> String {
        return "STK contract cont_type".to_string();
    }
}

