use crate::traits::TraitTermDescription::TraitTermDescription;

pub type MarketObjectCodeOfRateReset = String;

// impl TraitTermDescription for MarketObjectCodeOfRateReset {
//     fn get_identifier(&self) -> &str {
//         "marketObjectCodeOfRateReset"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Market Object Code Of Rate Reset"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRMO"
//     }
//     fn get_type(&self) -> &str {
//         "Varchar"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Is pointing to the interest rate driver (MarketObject) used for rate reset uniquely.
// Unique codes for market objects must be used."
//     }
// }    