use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type XDayNotice = IsoDuration;

// impl TraitTermDescription for XDayNotice {
//     fn get_identifier(&self) -> &str {
//         "xDayNotice"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "X Day Notice"
//     }
//     fn get_acronym(&self) -> &str {
//         "XDN"
//     }
//     fn get_type(&self) -> &str {
//         "Period"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Duration']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Used as rolling attribute with the callable CT's UMP and CLM uniquely. CLM's and UMP's will not be settled (MD not set) until the client uses his option to call the contract X_Day_Notice after Current Date. As long as MD or TD is not set, the client postpones his right to call to the future. The cycle is normally defined in number of business days."
//     }
// }   