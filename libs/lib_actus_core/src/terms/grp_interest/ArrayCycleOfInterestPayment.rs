use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Default, PartialEq)]
pub struct ArrayCycleOfInterestPayment {
    list_value: Vec<IsoDuration>
}

impl TraitTermDescription for ArrayCycleOfInterestPayment {
    fn get_identifier(&self) -> &str {
        "arrayCycleOfInterestPayment"
    }
    fn get_group(&self) -> &str {
        "Interest"
    }
    fn get_name(&self) -> &str {
        "Array Cycle Of Interest Payment"
    }
    fn get_acronym(&self) -> &str {
        "ARIPCLi"
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
        "Same like IPCL but as array"
    }
}