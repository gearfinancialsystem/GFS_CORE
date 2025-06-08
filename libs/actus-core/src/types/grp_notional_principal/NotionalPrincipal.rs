use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type NotionalPrincipal = RealPositive;

// impl TermDescriptionTrait for NotionalPrincipal {
//     fn get_identifier(&self) -> &str {
//         "notionalPrincipal"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_acronym(&self) -> &str {
//         "NT"
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
//         "Current nominal value of the contract. For debt instrument this is the current remaining notional outstanding. 
// NT is generally the basis on which interest payments are calculated. If IPCBS is set, IPCBS may introduce a different basis for interest payment calculation."
//     }
// }  
