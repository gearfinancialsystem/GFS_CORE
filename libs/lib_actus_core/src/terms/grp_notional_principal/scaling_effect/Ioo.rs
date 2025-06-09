use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct IOO;

impl IOO {
    pub fn new() -> Self {
        return IOO;
    }
    pub fn type_str(&self) -> String {
        return "IOO Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for IOO {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "interestIsScaled"
    }
    fn get_name(&self) -> &str {
        "Interest is Scaled"
    }
    fn get_acronym(&self) -> &str {
        "I00"
    }
    fn get_description(&self) -> &str {
        "Scaling applies only to interest."
    }
}    