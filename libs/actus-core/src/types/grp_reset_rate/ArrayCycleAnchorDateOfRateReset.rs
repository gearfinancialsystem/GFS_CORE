use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ArrayCycleAnchorDateOfRateReset {
    list_value: IsoDatetime,
}

impl TermDescriptionTrait for ArrayCycleAnchorDateOfRateReset {
    fn get_identifier(&self) -> &str {
        "arrayCycleAnchorDateOfRateReset"
    }
    fn get_group(&self) -> &str {
        "Rate Reset"
    }
    fn get_name(&self) -> &str {
        "Array Cycle Anchor Date Of Rate Reset"
    }
    fn get_acronym(&self) -> &str {
        "ARRRANX"
    }
    fn get_type(&self) -> &str {
        "Timestamp[]"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Same like RRANX but as array"
    }
}    
