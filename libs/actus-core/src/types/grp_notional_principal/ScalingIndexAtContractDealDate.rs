use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type ScalingIndexAtContractDealDate = f64;

// impl TermDescriptionTrait for ScalingIndexAtContractDealDate {
//     fn get_identifier(&self) -> &str {
//         "scalingIndexAtContractDealDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Scaling Index At Contract Deal Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "SCCDD"
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
//         "The value of the Scaling Index as per Contract Deal Date."
//     }
// }    