use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleAnchorDateOfRateReset = IsoDatetime;

// impl TermDescriptionTrait for CycleAnchorDateOfRateReset {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfRateReset"
//     }
//     fn get_group(&self) -> &str {
//         "Rate Reset"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Rate Reset"
//     }
//     fn get_acronym(&self) -> &str {
//         "RRANX"
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
//         "Date from which the rate reset date schedule is calculated according to the cycle length. The first rate reset event takes place on this anchor."
//     }
// }    