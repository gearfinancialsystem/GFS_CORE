use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct N;

impl N {
    pub fn new() -> Self {
        return N;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for N {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "noPrepayment"
    }
    fn get_name(&self) -> &str {
        "No Prepayment"
    }
    fn get_acronym(&self) -> &str {
        "N"
    }
    fn get_description(&self) -> &str {
        "Prepayment is not allowed under the agreement."
    }
}