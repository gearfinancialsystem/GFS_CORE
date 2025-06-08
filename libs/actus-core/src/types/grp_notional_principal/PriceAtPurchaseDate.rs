use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type PriceAtPurchaseDate = f64;

// impl TermDescriptionTrait for PriceAtPurchaseDate {
//     fn get_identifier(&self) -> &str {
//         "priceAtPurchaseDate"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Price At Purchase Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "PPRD"
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
//         "Purchase price exchanged at PRD.  
// PPRD represents a clean price (includes premium/discount but not IPAC)."
//     }
// }    