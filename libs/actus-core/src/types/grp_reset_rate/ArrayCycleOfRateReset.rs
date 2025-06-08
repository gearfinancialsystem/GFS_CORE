use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

#[derive(Debug, PartialEq)]
pub struct ArrayCycleOfRateReset {
    list_value: IsoDuration,
}

impl TermDescriptionTrait for ArrayCycleOfRateReset {
    fn get_identifier(&self) -> &str {
        "arrayCycleOfRateReset"
    }
    fn get_group(&self) -> &str {
        "Rate Reset"
    }
    fn get_name(&self) -> &str {
        "Array Cycle Of Rate Reset"
    }
    fn get_acronym(&self) -> &str {
        "ARRRCL"
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
        "Same like RRCL but as array"
    }
}
