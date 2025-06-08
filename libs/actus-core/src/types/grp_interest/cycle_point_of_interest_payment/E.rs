use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct E;

impl E {
    pub fn new() -> Self {
        return E;
    }
    pub fn type_str(&self) -> String {
        return "E Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for E {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "end"
    }
    fn get_name(&self) -> &str {
        "End"
    }
    fn get_acronym(&self) -> &str {
        "E"
    }
    fn get_description(&self) -> &str {
        "Interest is paid at the end of the interest period."
    }
}