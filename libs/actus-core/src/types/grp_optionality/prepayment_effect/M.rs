use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct M;

impl M {
    pub fn new() -> Self {
        return M;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for M {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "prepaymentReducesMaturity"
    }
    fn get_name(&self) -> &str {
        "Prepayment Reduces Maturity"
    }
    fn get_acronym(&self) -> &str {
        "M"
    }
    fn get_description(&self) -> &str {
        "Prepayment is allowed and reduces the maturity."
    }
}