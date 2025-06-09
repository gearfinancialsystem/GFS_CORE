use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CreditLineAmount = RealPositive;

// impl TraitTermDescription for CreditLineAmount {
//     fn get_identifier(&self) -> &str {
//         "creditLineAmount"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Credit Line Amount"
//     }
//     fn get_acronym(&self) -> &str {
//         "CLA"
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
//         "If defined, gives the total amount that can be drawn from a credit line. The remaining amount that can still be drawn is given by CLA-NT.
// For ANN, NAM, the credit line can only be drawn prior to PRANX-1PRCL.
// For CRL, the remaining amount that can still be drawn is given by CLA-Sum(NT of attached contracts)."
//     }
// }