use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type PriceAtTerminationDate = f64;

// impl TermDescriptionTrait for PriceAtTerminationDate {
//     fn get_identifier(&self) -> &str {
//         "priceAtTerminationDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Price At Termination Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "PTD"
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
//         "Sellingprice exchanged at PTD  PTDrepresents a clean price (includes premium/discount but not IPAC"
//     }
// }   