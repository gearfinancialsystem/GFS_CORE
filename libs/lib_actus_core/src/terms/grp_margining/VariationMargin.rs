use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type VariationMargin = RealPositive;


// impl TraitTermDescription for VariationMargin {
//     fn get_identifier(&self) -> &str {
//         "variationMargin"
//     }
//     fn get_group(&self) -> &str {
//         "Margining"
//     }
//     fn get_name(&self) -> &str {
//         "Variation Margin"
//     }
//     fn get_acronym(&self) -> &str {
//         "MRVM"
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
//         "MRVM reflects the accrued but not yet paid margin as per SD.  
// Open traded positions are revalued by the exchange at the end of every trading day using mark-to-market valuation. Often clearing members do not credit or debit their clients daily with MRVM, but rather use a Maintenance Margin. If the balance falls outside MRMML (and MRMMU), then  capital must be added (is refunded) to reach the original margin amount MRIM. We can also say that MVO+MRVM is equal to the reference value as per last margin update."
//     }
// }   