use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type InitialExchangeDate = NaiveDateTime;

// impl TraitTermDescription for InitialExchangeDate {
//     fn get_identifier(&self) -> &str {
//         "initialExchangeDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Initial Exchange Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "IED"
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
//         "Date of the initial cash flow for Maturity and Non-Maturity CT's. It also coincides with the beginning of interest accrual calculation."
//     }
// }  
