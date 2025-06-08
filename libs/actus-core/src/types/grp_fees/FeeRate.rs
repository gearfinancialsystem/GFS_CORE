use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type FeeRate = f64;

// impl TermDescriptionTrait for FeeRate {
//     fn get_identifier(&self) -> &str {
//         "feeRate"
//     }
//     fn get_group(&self) -> &str {
//         "Fees"
//     }
//     fn get_name(&self) -> &str {
//         "Fee Rate"
//     }
//     fn get_acronym(&self) -> &str {
//         "FER"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Rate of Fee which is a percentage of the underlying or FER is an absolute amount. For all contracts where FEB does not apply (cf. business rules), FER is interpreted as an absolute amount."
//     }
// }    