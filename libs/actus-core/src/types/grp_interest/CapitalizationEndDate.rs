use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CapitalizationEndDate = IsoDatetime;

// impl TermDescriptionTrait for CapitalizationEndDate {
//     fn get_identifier(&self) -> &str {
//         "capitalizationEndDate"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Capitalization End Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPCED"
//     }
//     fn get_type(&self) -> &str {
//         "Timestamp"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Datetime']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "If IPCED is set, then interest is not paid or received but added to the balance (NT) until IPCED. If IPCED does not coincide with an IP cycle, one additional interest payment gets calculated at IPCED and capitalized. Thereafter normal interest payments occur."
//     }
// }   