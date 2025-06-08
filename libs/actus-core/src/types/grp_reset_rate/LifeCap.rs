use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type LifeCap = f64;

// impl TermDescriptionTrait for LifeCap {
//     fn get_identifier(&self) -> &str {
//         "lifeCap"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Life Cap"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRLC"
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
//         "For variable rate basic CTs this represents a cap on the interest rate that applies during the entire lifetime of the contract.
// For CAPFL CTs this represents the cap strike rate."
//     }
// }    