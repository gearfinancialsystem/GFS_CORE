use crate::traits::TraitTermDescription::TraitTermDescription;

pub type FeeAccrued = f64;

// impl TraitTermDescription for FeeAccrued {
//     fn get_identifier(&self) -> &str {
//         "feeAccrued"
//     }
//     fn get_group(&self) -> &str {
//         "Fees"
//     }
//     fn get_name(&self) -> &str {
//         "Fee Accrued"
//     }
//     fn get_acronym(&self) -> &str {
//         "FEAC"
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
//         "Accrued fees as per SD"
//     }
// }    