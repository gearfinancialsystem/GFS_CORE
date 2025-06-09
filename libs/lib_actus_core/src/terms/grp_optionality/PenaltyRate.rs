use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type PenaltyRate = RealPositive;


// impl TraitTermDescription for PenaltyRate {
//     fn get_identifier(&self) -> &str {
//         "penaltyRate"
//     }
//     fn get_group(&self) -> &str {
//         "Optionality"
//     }
//     fn get_name(&self) -> &str {
//         "Penalty Rate"
//     }
//     fn get_acronym(&self) -> &str {
//         "PYRT"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['Positive']"
//     }
//     fn get_default_value(&self) -> &str {
//         "0"
//     }
//     fn get_description(&self) -> &str {
//         "Either the rate or the absolute amount of the prepayment."
//     }
// }   