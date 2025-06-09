use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type OptionStrike2 = RealPositive;

// impl TraitTermDescription for OptionStrike2 {
//     fn get_identifier(&self) -> &str {
//         "optionStrike2"
//     }
//     fn get_group(&self) -> &str {
//         "Optionality"
//     }
//     fn get_name(&self) -> &str {
//         "Option Strike 2"
//     }
//     fn get_acronym(&self) -> &str {
//         "sss"
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
//         "Put price in case of call/put."
//     }
// }    