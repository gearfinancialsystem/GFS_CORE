use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type NextDividendPaymentAmount = RealPositive;

// impl TermDescriptionTrait for NextDividendPaymentAmount {
//     fn get_identifier(&self) -> &str {
//         "nextDividendPaymentAmount"
//     }
//     fn get_group(&self) -> &str {
//         "Dividend"
//     }
//     fn get_name(&self) -> &str {
//         "Next Dividend Payment Amount"
//     }
//     fn get_acronym(&self) -> &str {
//         "DVNP"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['Positive']"
//     }
//     fn get_default_value(&self) -> &str {
//         "0"
//     }
//     fn get_description(&self) -> &str {
//         "Defines the next dividend payment (amount) whereas the date of dividend payment is defined through the DVANX/DVCL pair. If DVCL is defined, then this amount will be used as dividend payment for each future dividend payment date."
//     }
// }   