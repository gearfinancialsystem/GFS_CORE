use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type TerminationDate = IsoDatetime;

// impl TermDescriptionTrait for TerminationDate {
//     fn get_identifier(&self) -> &str {
//         "terminationDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Termination Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "TD"
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
//         "If a contract is sold before MD (for example a bond on the secondary market) this date has to be set. It refers to the date at which the payment (of PTD) and transfer of the security happens. In other words, TD - if set - takes the cont_role otherwise MD has from a cash flow perspective. 
// Note, CPID of the CT is not the counterparty of the transaction!"
//     }
// }    