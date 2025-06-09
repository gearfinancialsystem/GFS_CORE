use crate::traits::TraitTermDescription::TraitTermDescription;

pub type RateSpread = f64;

// impl TraitTermDescription for RateSpread {
//     fn get_identifier(&self) -> &str {
//         "rateSpread"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Rate Spread"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRSP"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         "0"
//     }
//     fn get_description(&self) -> &str {
//         "Interest rate spread. A typical rate resetting rule is LIBOR plus x basis point where x represents the interest rate spread.  
// The following equation can be taken if RRMLT is not set: IPNR after rate reset = Rate selected from the market object  + RRSP."
//     }
// }    