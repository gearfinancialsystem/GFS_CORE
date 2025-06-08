use crate::subtypes::RealPositive::RealPositive;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type MaximumPenaltyFreeDisbursement = RealPositive;

// impl TermDescriptionTrait for MaximumPenaltyFreeDisbursement {
//     fn get_identifier(&self) -> &str {
//         "maximumPenaltyFreeDisbursement"
//     }
//     fn get_group(&self) -> &str {
//         "Optionality"
//     }
//     fn get_name(&self) -> &str {
//         "Maximum Penalty Free Disbursement"
//     }
//     fn get_acronym(&self) -> &str {
//         "MPFD"
//     }
//     fn get_type(&self) -> &str {
//         "Real"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['Positive']"
//     }
//     fn get_default_value(&self) -> &str {
//         "[ the value of notionalPrincipal ]"
//     }
//     fn get_description(&self) -> &str {
//         "Defines the notional amount which can be withdrawn before XDN without penalty"
//     }
// }    