use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ArrayCycleAnchorDateOfInterestPayment {
    list_value: Vec<IsoDatetime>
}

impl TermDescriptionTrait for ArrayCycleAnchorDateOfInterestPayment {
    fn get_identifier(&self) -> &str {
        "arrayCycleAnchorDateOfInterestPayment"
    }
    fn get_group(&self) -> &str {
        "Interest"
    }
    fn get_name(&self) -> &str {
        "Array Cycle Anchor Date Of Interest Payment"
    }
    fn get_acronym(&self) -> &str {
        "ARIPANXi"
    }
    fn get_type(&self) -> &str {
        "Timestamp[]"
    }
    fn get_allowed_values(&self) -> &str {
        "['ISO8601 Datetime']"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Same like IPANX but as array"
    }
}    