use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleOfFee = IsoDuration;
//// a completer avec la contrainte

// impl TermDescriptionTrait for CycleOfFee {
//     fn get_identifier(&self) -> &str {
//         "cycleOfFee"
//     }
//     fn get_group(&self) -> &str {
//         "Fees"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Fee"
//     }
//     fn get_acronym(&self) -> &str {
//         "FECL"
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
//         "Defines in combination with FEANX the payment points of fees"
//     }
// }    