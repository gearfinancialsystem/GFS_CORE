use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type NotionalScalingMultiplier = f64;

// impl TermDescriptionTrait for NotionalScalingMultiplier {
//     fn get_identifier(&self) -> &str {
//         "notionalScalingMultiplier"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Notional Scaling Multiplier"
//     }
//     fn get_acronym(&self) -> &str {
//         "SCNT"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         "1"
//     }
//     fn get_description(&self) -> &str {
//         "The multiplier being applied to principal cash flows"
//     }
// }    
