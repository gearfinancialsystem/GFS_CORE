use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct B;

impl B {
    pub fn new() -> Self {
        return B;
    }
    pub fn type_str(&self) -> String {
        return "B Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for B {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "beginning"
    }
    fn get_name(&self) -> &str {
        "Beginning"
    }
    fn get_acronym(&self) -> &str {
        "B"
    }
    fn get_description(&self) -> &str {
        "Interest is paid upfront of the interest period."
    }
}