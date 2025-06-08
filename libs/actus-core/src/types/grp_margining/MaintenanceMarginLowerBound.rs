use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type MaintenanceMarginLowerBound = RealPositive;

// impl TermDescriptionTrait for MaintenanceMarginLowerBound {
//     fn get_identifier(&self) -> &str {
//         "maintenanceMarginLowerBound"
//     }
//     fn get_group(&self) -> &str {
//         "Margining"
//     }
//     fn get_name(&self) -> &str {
//         "Maintenance Margin Lower Bound"
//     }
//     fn get_acronym(&self) -> &str {
//         "MRMML"
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
//         "Defines the lower bound of the Maintenance Margin. If MRVM falls below MRMML, then capital must be added to reach the original MRIM."
//     }
// }    