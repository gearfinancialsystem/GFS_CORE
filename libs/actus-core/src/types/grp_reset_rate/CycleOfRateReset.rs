use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleOfRateReset = IsoDuration;

// impl TermDescriptionTrait for CycleOfRateReset {
//     fn get_identifier(&self) -> &str {
//         "cycleOfRateReset"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Rate Reset"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRCL"
//     }
//     fn get_type(&self) -> &str {
//         "Cycle"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['[ISO8601 Duration]L[s={0,1}]']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Cycle according to which the rate reset date schedule will be calculated.
// In case RRCL is not set, then there will only be one rate reset event at RRANX given RRANX if set.
// The interval will be adjusted yet by EOMC and BDC."
//     }
// }   