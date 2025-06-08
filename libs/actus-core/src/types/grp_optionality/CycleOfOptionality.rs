use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleOfOptionality = IsoDuration;

// impl TermDescriptionTrait for CycleOfOptionality {
//     fn get_identifier(&self) -> &str {
//         "cycleOfOptionality"
//     }
//     fn get_group(&self) -> &str {
//         "Optionality"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Optionality"
//     }
//     fn get_acronym(&self) -> &str {
//         "OPCL"
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
//         "Cycle according to which the option exercise date schedule will be calculated.
// OPCL can be NULL for American Options or Prepayment Optionality in which case the optionality period starts at OPANX and ends at OPXED (for american options) or MD (in case of prepayment optionality).
// The interval will be adjusted yet by EOMC and BDC."
//     }
// }    