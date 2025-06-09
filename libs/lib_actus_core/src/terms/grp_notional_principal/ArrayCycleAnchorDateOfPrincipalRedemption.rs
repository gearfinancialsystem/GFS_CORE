use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ArrayCycleAnchorDateOfPrincipalRedemption {
    list: Vec<NaiveDateTime>
}


impl TraitTermDescription for ArrayCycleAnchorDateOfPrincipalRedemption {
    fn get_identifier(&self) -> &str {
        "arrayCycleAnchorDateOfPrincipalRedemption"
    }
    fn get_group(&self) -> &str {
        "Notional Principal"
    }
    fn get_name(&self) -> &str {
        "Array Cycle Anchor Date Of Principal Redemption"
    }
    fn get_acronym(&self) -> &str {
        "ARPRANXj"
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
        "Same like PRANX but as array"
    }
}   