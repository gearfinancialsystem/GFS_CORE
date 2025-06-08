use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type InitialMargin = RealPositive;

// impl TermDescriptionTrait for InitialMargin {
//     fn get_identifier(&self) -> &str {
//         "initialMargin"
//     }
//     fn get_group(&self) -> &str {
//         "Margining"
//     }
//     fn get_name(&self) -> &str {
//         "Initial Margin"
//     }
//     fn get_acronym(&self) -> &str {
//         "MRIM"
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
//         "Margin to cover losses which may be incurred as a result of market fluctuations. 
// Upon contract closing or maturity, the MRIM is reimbursed."
//     }
// }   