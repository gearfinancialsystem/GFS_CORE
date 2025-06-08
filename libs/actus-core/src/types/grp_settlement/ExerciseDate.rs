use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type ExerciseDate = IsoDatetime;

// impl TermDescriptionTrait for ExerciseDate {
//     fn get_identifier(&self) -> &str {
//         "exerciseDate"
//     }
//     fn get_group(&self) -> &str {
//         "Settlement"
//     }
//     fn get_name(&self) -> &str {
//         "Exercise Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "XD"
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
//         "Date of exercising a contingent event/obligation such as a forward condition, optionality etc. The Exercise date marks the observed timestamp of fixing the contingent event and respective payment obligation not necessarily the timestamp of settling the obligation."
//     }
// }  