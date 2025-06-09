use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type NonPerformingDate = NaiveDateTime;

// impl TraitTermDescription for NonPerformingDate {
//     fn get_identifier(&self) -> &str {
//         "nonPerformingDate"
//     }
//     fn get_group(&self) -> &str {
//         "Counterparty"
//     }
//     fn get_name(&self) -> &str {
//         "Non Performing Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "NPD"
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
//         "The date of the (uncovered) payment event responsible for the current value of the Contract Performance attribute."
//     }
// }    