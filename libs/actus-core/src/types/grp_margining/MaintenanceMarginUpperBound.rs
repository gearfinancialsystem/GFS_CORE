use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type MaintenanceMarginUpperBound = RealPositive;

// impl TermDescriptionTrait for MaintenanceMarginUpperBound {
//     fn get_identifier(&self) -> &str {
//         "maintenanceMarginUpperBound"
//     }
//     fn get_group(&self) -> &str {
//         "Margining"
//     }
//     fn get_name(&self) -> &str {
//         "Maintenance Margin Upper Bound"
//     }
//     fn get_acronym(&self) -> &str {
//         "MRMMU"
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
//         "Defines the upper bound of the Maintenance Margin. If MRVM falls above MRMMU, then capital is refunded to reach the original MRIM."
//     }
// }