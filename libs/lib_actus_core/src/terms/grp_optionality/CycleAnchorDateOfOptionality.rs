use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleAnchorDateOfOptionality = NaiveDateTime;

// impl TraitTermDescription for CycleAnchorDateOfOptionality {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfOptionality"
//     }
//     fn get_group(&self) -> &str {
//         "Optionality"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Optionality"
//     }
//     fn get_acronym(&self) -> &str {
//         "OPANX"
//     }
//     fn get_type(&self) -> &str {
//         "Timestamp"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Datetime']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Used for Basic Maturities (such as PAM, RGM, ANN, NGM and their Step-up versions) and American and Bermudan style options. 
// - Basic Maturities: Within the group of these Maturities, it indicates the possibility of prepayments. Prepayment features are controlled by Behavior. 
// - American and Bermudan style Options: Begin of exercise period."
//     }
// }    