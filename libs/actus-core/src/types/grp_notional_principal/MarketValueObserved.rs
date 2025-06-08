use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type MarketValueObserved = f64;

// impl TermDescriptionTrait for MarketValueObserved {
//     fn get_identifier(&self) -> &str {
//         "marketValueObserved"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Market Value Observed"
//     }
//     fn get_acronym(&self) -> &str {
//         "MVO"
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
//         "Value as observed in the market at SD per unit. Incase of fixed income instruments it is a fraction."
//     }
// }    