

#[derive(Debug, Eq, PartialEq)]
pub struct LAX;

impl LAX {
    pub fn new() -> Self {
        return LAX;
    }
    pub fn type_str(&self) -> String {
        return "LAX contract cont_type".to_string();
    }
}

impl TraitEnumOptionDescription for LAX {
    fn get_option_rank(&self) -> &str {
        "4"
    }
    fn get_identifier(&self) -> &str {
        "exoticLinearAmortizer"
    }
    fn get_name(&self) -> &str {
        "Exotic Linear Amortizer"
    }
    fn get_acronym(&self) -> &str {
        "LAX"
    }
    fn get_description(&self) -> &str {
        "Lending agreements with exotic repayment schedules."
    }
}    