use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct ONO;

impl ONO {
    pub fn new() -> Self {
        return ONO;
    }
    pub fn type_str(&self) -> String {
        return "ONO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for ONO {
fn get_option_rank(&self) -> &str {
"2"
}
fn get_identifier(&self) -> &str {
"principalIsScaled"
}
fn get_name(&self) -> &str {
"Principal is Scaled"
}
fn get_acronym(&self) -> &str {
"0N0"
}
fn get_description(&self) -> &str {
               "Scaling applies only to principal."
}
}    