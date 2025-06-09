use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type GracePeriod = IsoDuration;

// 
// impl TraitTermDescription for GracePeriod {
//     fn get_identifier(&self) -> &str {
//         "gracePeriod"
//     }
//     fn get_group(&self) -> &str {
//         "Counterparty"
//     }
//     fn get_name(&self) -> &str {
//         "Grace Period"
//     }
//     fn get_acronym(&self) -> &str {
//         "GRP"
//     }
//     fn get_type(&self) -> &str {
//         "Period"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Duration']"
//     }
//     fn get_default_value(&self) -> &str {
//         "P0D"
//     }
//     fn get_description(&self) -> &str {
//         "If real payment happens after scheduled payment date plus GRP, then the payment is in delay."
//     }
// }    