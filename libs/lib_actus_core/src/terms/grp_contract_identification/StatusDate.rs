
use chrono::NaiveDateTime;

pub type StatusDate = NaiveDateTime;

// impl TraitTermDescription for StatusDate {
//     fn get_identifier(&self) -> &str {
//         "statusDate"
//     }
//     fn get_group(&self) -> &str {
//         "Contract identification"
//     }
//     fn get_name(&self) -> &str {
//         "Status Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "SD"
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
//         "SD holds the date per which all attributes of the record were updated. This is especially important for the highly dynamic attributes like Accruals, Notional, interest rates in variable instruments etc."
//     }
// }