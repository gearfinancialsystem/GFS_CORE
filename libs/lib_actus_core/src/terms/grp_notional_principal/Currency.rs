use crate::traits::TraitTermDescription::TraitTermDescription;

pub type Currency = String;

//// a modifier

// impl TraitTermDescription for Currency {
//     fn get_identifier(&self) -> &str {
//         "currency"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Currency"
//     }
//     fn get_acronym(&self) -> &str {
//         "CUR"
//     }
//     fn get_type(&self) -> &str {
//         "Varchar"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO4217']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "The currency of the cash flows."
//     }
// }   