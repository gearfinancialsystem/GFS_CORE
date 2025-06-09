use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type OptionExerciseEndDate = NaiveDateTime;

// impl TraitTermDescription for OptionExerciseEndDate {
//     fn get_identifier(&self) -> &str {
//         "optionExerciseEndDate"
//     }
//     fn get_group(&self) -> &str {
//         "Optionality"
//     }
//     fn get_name(&self) -> &str {
//         "Option Exercise End Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "OPXED"
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
//         "Final exercise date for American and Bermudan options, expiry date for European options."
//     }
// }    
