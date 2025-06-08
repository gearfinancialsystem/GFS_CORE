use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type PeriodFloor = RealPositive;


// impl TermDescriptionTrait for PeriodFloor {
//     fn get_identifier(&self) -> &str {
//         "periodFloor"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Period Floor"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRPF"
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
//         "For variable rate basic CTs this represents the maximum negative rate change per rate reset cycle."
//     }
// }
