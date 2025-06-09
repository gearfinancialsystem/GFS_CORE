use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleOfInterestCalculationBase = IsoDuration;

// impl TraitTermDescription for CycleOfInterestCalculationBase {
//     fn get_identifier(&self) -> &str {
//         "cycleOfInterestCalculationBase"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Interest Calculation Base"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPCBCL"
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
//         "Concerning the format see PRCL.
// Defines the subsequent adjustment points to NT of the interest payment calculation base."
//     }
// }