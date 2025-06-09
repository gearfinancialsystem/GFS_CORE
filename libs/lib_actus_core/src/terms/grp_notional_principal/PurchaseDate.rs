use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type PurchaseDate = NaiveDateTime;

// impl TraitTermDescription for PurchaseDate {
//     fn get_identifier(&self) -> &str {
//         "purchaseDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Purchase Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "PRD"
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
//         "If a contract is bought after initiation (for example a bond on the secondary market) this date has to be set. It refers to the date at which the payment (of PPRD) and transfer of the security happens. In other words, PRD - if set - takes the cont_role otherwise IED has from a cash flow perspective. 
// Note, CPID of the CT is not the counterparty of the transaction!"
//     }
// }    