use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type OptionStrike1 = RealPositive;

// impl TraitTermDescription for OptionStrike1 {
//     fn get_identifier(&self) -> &str {
//         "optionStrike1"
//     }
//     fn get_group(&self) -> &str {
//         "Optionality"
//     }
//     fn get_name(&self) -> &str {
//         "Option Strike 1"
//     }
//     fn get_acronym(&self) -> &str {
//         "OPS1"
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
//         "Strike price of the option. Whether it is a call/put is determined by the attribute OPTP, i.e a call or a put (or a combination of call/put).
// This attribute is used for price related options such as options on bonds, stocks or FX. Interest rate related options (caps/floos) are handled within th RatReset group."
//     }
// }    
//     