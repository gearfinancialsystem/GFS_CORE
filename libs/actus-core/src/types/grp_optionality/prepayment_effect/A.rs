use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]

pub struct A;

impl A {
    pub fn new() -> Self {
        return A;
    }
    pub fn type_str(&self) -> String {
        return "A Scaling Effect".to_string();
    }
}

impl TraitEnumOptionDescription for A {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "prepaymentReducesRedemptionAmount"
    }
    fn get_name(&self) -> &str {
        "Prepayment Reduces Redemption Amount"
    }
    fn get_acronym(&self) -> &str {
        "A"
    }
    fn get_description(&self) -> &str {
        "Prepayment is allowed and reduces the redemption amount for the remaining period up to maturity."
    }
}