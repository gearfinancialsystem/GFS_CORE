use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleAnchorDateOfFee = NaiveDateTime;

// impl TraitTermDescription for CycleAnchorDateOfFee {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfFee"
//     }
//     fn get_group(&self) -> &str {
//         "Fees"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Fee"
//     }
//     fn get_acronym(&self) -> &str {
//         "FEANX"
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
//         "Date from which the fee payment date schedule is calculated according to the cycle length. The first fee payment event takes place on this anchor."
//     }
// }  