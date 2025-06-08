use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type ExDividendDate = IsoDatetime;

// impl TermDescriptionTrait for ExDividendDate {
//     fn get_identifier(&self) -> &str {
//         "exDividendDate"
//     }
//     fn get_group(&self) -> &str {
//         "Dividend"
//     }
//     fn get_name(&self) -> &str {
//         "Ex Dividend Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "DVEX"
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
//         "In case contract is traded between DVEX and next DV payment date (i.e. PRD>DVEX & PRD<next DV payment date), then the old holder of the contract (previous to the trade) receives the next DV payment. In other words, the next DV payment is cancelled for the new (after the trade) holder of the contract."
//     }
// }
