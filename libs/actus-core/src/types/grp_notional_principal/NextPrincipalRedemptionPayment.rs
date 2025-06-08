use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type NextPrincipalRedemptionPayment = RealPositive;

// impl TermDescriptionTrait for NextPrincipalRedemptionPayment {
//     fn get_identifier(&self) -> &str {
//         "nextPrincipalRedemptionPayment"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Next Principal Redemption Payment"
//     }
//     fn get_acronym(&self) -> &str {
//         "PRNXT"
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
//         "Amount of principal that will be paid during the redemption cycle at the next payment date. For amortizing contracts like ANN, NAM, ANX, and NAX this is the total periodic payment amount (sum of interest and principal)."
//     }
// }   