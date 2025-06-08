use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

#[derive(Debug, Default, PartialEq)]
pub struct ArrayCycleOfPrincipalRedemption {
    list: Vec<IsoDuration>
}


impl TermDescriptionTrait for ArrayCycleOfPrincipalRedemption {
    fn get_identifier(&self) -> &str {
        "arrayCycleOfPrincipalRedemption"
    }
    fn get_group(&self) -> &str {
        "Notional Principal"
    }
    fn get_name(&self) -> &str {
        "Array Cycle Of Principal Redemption"
    }
    fn get_acronym(&self) -> &str {
        "ARPRCLj"
    }
    fn get_type(&self) -> &str {
        "Cycle[]"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Same like PRCL but as array"
    }
}   