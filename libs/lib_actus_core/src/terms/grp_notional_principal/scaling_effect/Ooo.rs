use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct OOO;

impl OOO {
    pub fn new() -> Self {
        return OOO;
    }
    pub fn type_str(&self) -> String {
        return "OOO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for OOO {
fn get_option_rank(&self) -> &str {
"0"
}
fn get_identifier(&self) -> &str {
"noScaling"
}
fn get_name(&self) -> &str {
"No Scaling"
}
fn get_acronym(&self) -> &str {
"000"
}
fn get_description(&self) -> &str {
"No scaling applies."
}
}    