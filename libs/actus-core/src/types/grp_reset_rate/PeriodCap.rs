use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type PeriodCap = RealPositive;

// impl TermDescriptionTrait for PeriodCap {
//     fn get_identifier(&self) -> &str {
//         "periodCap"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Period Cap"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRPC"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['Positive']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "For variable rate basic CTs this represents the maximum positive rate change per rate reset cycle."
//     }
// }    