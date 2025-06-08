use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type FuturesPrice = f64;

// impl TermDescriptionTrait for FuturesPrice {
//     fn get_identifier(&self) -> &str {
//         "futuresPrice"
//     }
//     fn get_group(&self) -> &str {
//         "Settlement"
//     }
//     fn get_name(&self) -> &str {
//         "Futures Price"
//     }
//     fn get_acronym(&self) -> &str {
//         "PFUT"
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
//         "The price the counterparties agreed upon at which the underlying contract (of a FUTUR) is exchanged/settled at STD. Quoting is different for different types of underlyings: Fixed Income = in percentage, all others in nominal terms."
//     }
// }    