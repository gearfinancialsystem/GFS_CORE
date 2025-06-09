use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct NAM;

impl NAM {
    pub fn new() -> Self {
        return NAM;
    }
    pub fn type_str(&self) -> String {
        return "NAM contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for NAM {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "negativeAmortizer"
    }
    fn get_name(&self) -> &str {
        "Negative Amortizer"
    }
    fn get_acronym(&self) -> &str {
        "NAM"
    }
    fn get_description(&self) -> &str {
        "Lending agreements with fixed periodic payments consisting of an interest and principal portion. Maturity changes for variable rate instruments. "
    }
}    