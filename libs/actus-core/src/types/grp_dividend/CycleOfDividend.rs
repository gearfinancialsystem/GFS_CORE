use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleOfDividend = IsoDuration;
//// a compléter

// impl TermDescriptionTrait for CycleOfDividend {
//     fn get_identifier(&self) -> &str {
//         "cycleOfDividend"
//     }
//     fn get_group(&self) -> &str {
//         "Dividend"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Dividend"
//     }
//     fn get_acronym(&self) -> &str {
//         "DVCL"
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
//         "Defines in combination with DVANX the payment points of dividends. The dividend payment schedule will start at DVANX and end at MaximumProjectionPeriod (cf. sheet Modeling Parameters)."
//     }
// }    