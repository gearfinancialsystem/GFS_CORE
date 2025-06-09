use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type ContractDealDate = NaiveDateTime;

// impl TraitTermDescription for ContractDealDate {
//     fn get_identifier(&self) -> &str {
//         "contractDealDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Contract Deal Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "CDD"
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
//         "This date signifies the origination of the contract where an agreement between the customer and the bank has been settled. From this date on, the institution will have a (market) risk position for financial contracts. This is even the case when IED is in future."
//     }
// }   