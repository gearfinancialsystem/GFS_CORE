use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct DQ;

impl DQ {
    pub fn new() -> Self {
        return DQ;
    }
    pub fn type_str(&self) -> String {
        return "DQ contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for DQ {
    fn get_option_rank(&self) -> &str {
        "2"
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
        "Contractual payment obligations are delinquent according to the Delinquency Period."
    }
}   