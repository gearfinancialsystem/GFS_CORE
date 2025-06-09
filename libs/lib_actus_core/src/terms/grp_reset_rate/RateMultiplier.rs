use crate::traits::TraitTermDescription::TraitTermDescription;

pub type RateMultiplier = f64;

// impl TraitTermDescription for RateMultiplier {
//     fn get_identifier(&self) -> &str {
//         "rateMultiplier"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Rate Multiplier"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRMLT"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "[]"
//     }
//     fn get_default_value(&self) -> &str {
//         "1"
//     }
//     fn get_description(&self) -> &str {
//         "Interest rate multiplier. A typical rate resetting rule is LIBOR plus x basis point where x represents the interest rate spread.
// However, in some cases like reverse or super floater contracts an additional rate multiplier applies. In this case, the new rate is determined as: IPNR after rate reset = Rate selected from the market object * RRMLT + RRSP."
//     }
// }    