use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct INO;

impl INO {
    pub fn new() -> Self {
        return INO;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}


impl TraitEnumOptionDescription for INO {
    fn get_option_rank(&self) -> &str {
        "3"
    }
    fn get_identifier(&self) -> &str {
        "interestAndPrincipalIsScaled"
    }
    fn get_name(&self) -> &str {
        "Interest and Principal is Scaled"
    }
    fn get_acronym(&self) -> &str {
        "IN0"
    }
    fn get_description(&self) -> &str {
        "Scaling applies to interest and principal."
    }
}    