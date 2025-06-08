use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type InterestScalingMultiplier = f64;

// impl TermDescriptionTrait for InterestScalingMultiplier {
//     fn get_identifier(&self) -> &str {
//         "interestScalingMultiplier"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Interest Scaling Multiplier"
//     }
//     fn get_acronym(&self) -> &str {
//         "SCIP"
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
//         "The multiplier being applied to interest cash flows"
//     }
// }   