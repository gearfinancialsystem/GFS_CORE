use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct CLM;
impl CLM {
    pub fn new() -> Self {
        return CLM;
    }
    pub fn type_str(&self) -> String {
        return "CLM contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for CLM {
    fn get_option_rank(&self) -> &str {
        "5"
    }
    fn get_identifier(&self) -> &str {
        "callMoney"
    }
    fn get_name(&self) -> &str {
        "Call Money"
    }
    fn get_acronym(&self) -> &str {
        "CLM"
    }
    fn get_description(&self) -> &str {
        "Lonas that are rolled over as long as they are not called. Once called it has to be paid back after the stipulated notice period."
    }
}