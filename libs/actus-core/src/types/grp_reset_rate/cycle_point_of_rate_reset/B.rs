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
    /// Return elements as defined in https://github.com/actusfrf/actus-dictionary/blob/master/actus-dictionary-terms.json
    fn get_option_rank(&self) -> &str {
        return "0";
    }
    fn get_identifier(&self) -> &str {
        return "beginning";
    }
    fn get_name(&self) -> &str {
        return "Beginning";
    }
    fn get_acronym(&self) -> &str {
        return "B";
    }
    fn get_description(&self) -> &str {
        "The new rate is applied at the beginning of the reset period."
    }
}