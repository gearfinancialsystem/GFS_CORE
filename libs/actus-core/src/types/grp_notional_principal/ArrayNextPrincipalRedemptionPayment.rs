use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

#[derive(Debug, Default, PartialEq)]
pub struct ArrayNextPrincipalRedemptionPayment {
    list_value: Vec<f64>
}

impl TermDescriptionTrait for ArrayNextPrincipalRedemptionPayment {
    fn get_identifier(&self) -> &str {
        "arrayNextPrincipalRedemptionPayment"
    }
    fn get_group(&self) -> &str {
        "Notional Principal"
    }
    fn get_name(&self) -> &str {
        "Array Next Principal Redemption Payment"
    }
    fn get_acronym(&self) -> &str {
        "ARPRNXTj"
    }
    fn get_type(&self) -> &str {
        "Real[]"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Same like PRNXT but as array"
    }
}    