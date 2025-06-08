use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type ExerciseAmount = f64;

// impl TermDescriptionTrait for ExerciseAmount {
//     fn get_identifier(&self) -> &str {
//         "exerciseAmount"
//     }
//     fn get_group(&self) -> &str {
//         "Settlement"
//     }
//     fn get_name(&self) -> &str {
//         "Exercise Amount"
//     }
//     fn get_acronym(&self) -> &str {
//         "XA"
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
//         "The amount fixed at Exercise Date for a contingent event/obligation such as a forward condition, optionality etc. The Exercise Amount is fixed at Exercise Date but not settled yet."
//     }
// }  