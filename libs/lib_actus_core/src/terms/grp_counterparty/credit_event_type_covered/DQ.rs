use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct DQ;

impl DQ {
    pub fn new() -> Self {
        return DQ;
    }
    pub fn type_str(&self) -> String {
        return "INO Scaling Effect".to_string();
    }
}
impl TraitEnumOptionDescription for DQ {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "delinquent"
    }
    fn get_name(&self) -> &str {
        "Delinquent"
    }
    fn get_acronym(&self) -> &str {
        "DQ"
    }
    fn get_description(&self) -> &str {
        "Delinquency of the underlying represents a credit event."
    }
}    