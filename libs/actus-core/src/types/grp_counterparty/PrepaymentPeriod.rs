use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type PrepaymentPeriod = IsoDuration;

// impl TermDescriptionTrait for PrepaymentPeriod {
//     fn get_identifier(&self) -> &str {
//         "prepaymentPeriod"
//     }
//     fn get_group(&self) -> &str {
//         "Counterparty"
//     }
//     fn get_name(&self) -> &str {
//         "Prepayment Period"
//     }
//     fn get_acronym(&self) -> &str {
//         "PPP"
//     }
//     fn get_type(&self) -> &str {
//         "Period"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Duration']"
//     }
//     fn get_default_value(&self) -> &str {
//         "P0D"
//     }
//     fn get_description(&self) -> &str {
//         "If real payment happens before scheduled payment date minus PPP, then it is considered a prepayment. Effect of prepayments are further described in PPEF and related fields."
//     }
// }    