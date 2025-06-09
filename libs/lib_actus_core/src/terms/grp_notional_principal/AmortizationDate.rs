use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type AmortizationDate = NaiveDateTime;

// impl TraitTermDescription for AmortizationDate {
//     fn get_identifier(&self) -> &str {
//         "amortizationDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Amortization Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "AMD"
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
//         "This Date is used to calculate the annuity amounts for ANN and ANX NGX CT's. Needs only to be set in case where the contract balloon at MD and MD is less than AD."
//     }
// }    