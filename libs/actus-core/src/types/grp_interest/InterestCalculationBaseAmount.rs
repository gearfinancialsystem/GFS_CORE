use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type InterestCalculationBaseAmount = RealPositive;

// impl TermDescriptionTrait for InterestCalculationBaseAmount {
//     fn get_identifier(&self) -> &str {
//         "interestCalculationBaseAmount"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Interest Calculation Base Amount"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPCBA"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['Positive']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "This is the amount used for the calculation of interest. Calculation base per SD."
//     }
// }    