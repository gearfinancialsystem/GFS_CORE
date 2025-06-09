use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type FixingPeriod = IsoDuration;

// impl TraitTermDescription for FixingPeriod {
//     fn get_identifier(&self) -> &str {
//         "fixingPeriod"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Fixing Period"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRFIX"
//     }
//     fn get_type(&self) -> &str {
//         "Period"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Duration']"
//     }
//     fn get_default_value(&self) -> &str {
//         "P0D"
//     }
//     fn get_description(&self) -> &str {
//         "Interest rate resets (adjustments) are usually fixed one or two days (usually Business Days) before the new rate applies (defined by the rate reset schedule). This field holds the period between fixing and application of a rate."
//     }
// }  