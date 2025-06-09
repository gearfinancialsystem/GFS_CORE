use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleOfMargining = IsoDuration;
////  a completer
// impl TraitTermDescription for CycleOfMargining {
//     fn get_identifier(&self) -> &str {
//         "cycleOfMargining"
//     }
//     fn get_group(&self) -> &str {
//         "Margining"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Margining"
//     }
//     fn get_acronym(&self) -> &str {
//         "MRCL"
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
//         "Defines together with MRANX the points where margins can be called."
//     }
// }    