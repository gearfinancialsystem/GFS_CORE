use crate::traits::TraitTermDescription::TraitTermDescription;

pub type AccruedInterest = f64;

// impl TraitTermDescription for AccruedInterest {
//     fn get_identifier(&self) -> &str {
//         "accruedInterest"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Accrued Interest"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPAC"
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
//         "Accrued interest as per SD. In case of NULL, this value will be recalculated using IPANX, IPCL and IPNR information. Can be used to represent irregular next IP payments."
//     }
// }    