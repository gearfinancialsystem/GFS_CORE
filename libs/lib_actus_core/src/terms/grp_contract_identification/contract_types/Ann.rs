use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct ANN;

impl ANN {
    pub fn new() -> Self {
        return ANN;
    }
    pub fn type_str(&self) -> String {
        return "ANN contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for ANN {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "annuity"
    }
    fn get_name(&self) -> &str {
        "Annuity"
    }
    fn get_acronym(&self) -> &str {
        "ANN"
    }
    fn get_description(&self) -> &str {
        "Lending agreements with fixed periodic payments consisting of an interest and principal portion. The periodic payments are adjusted for variable rate instruments such that maturity remains fixed."
    }
}

