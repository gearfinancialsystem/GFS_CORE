use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleAnchorDateOfPrincipalRedemption = IsoDatetime;


// impl TermDescriptionTrait for CycleAnchorDateOfPrincipalRedemption {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfPrincipalRedemption"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Principal Redemption"
//     }
//     fn get_acronym(&self) -> &str {
//         "PRANX"
//     }
//     fn get_type(&self) -> &str {
//         "Timestamp"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Datetime']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Date from which the principal payment date schedule is calculated according to the cycle length. The first principal payment event takes place on this anchor."
//     }
// }  