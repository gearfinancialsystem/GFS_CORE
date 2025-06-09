use crate::traits::TraitTermDescription::TraitTermDescription;

pub type NextResetRate = f64;

// impl TraitTermDescription for NextResetRate {
//     fn get_identifier(&self) -> &str {
//         "nextResetRate"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Next Reset Rate"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRNXT"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Holds the new rate that has been fixed already (cf. attribute FixingDays) but not applied. This new rate will be applied at the next rate reset event (after SD and according to the rate reset schedule). Attention, RRNXT must be set to NULL after it is applied!"
//     }
// }    