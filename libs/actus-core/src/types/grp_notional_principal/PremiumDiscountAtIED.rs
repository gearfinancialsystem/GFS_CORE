use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type PremiumDiscountAtIED = f64;

// impl TermDescriptionTrait for PremiumDiscountAtIED {
//     fn get_identifier(&self) -> &str {
//         "premiumDiscountAtIED"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Premium Discount At IED"
//     }
//     fn get_acronym(&self) -> &str {
//         "PDIED"
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
//         "Total original premium or discount that has been set at CDD and will be added to the (notional) cash flow at IED (cash flow at IED = NT+PDIED, w.r.t. an RPA CT). 
// Negative value for discount and positive for premium.
// Note, similar to interest the PDIED portion is part of P&L."
//     }
// }    

