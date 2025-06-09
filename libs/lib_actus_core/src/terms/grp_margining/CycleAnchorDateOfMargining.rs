use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleAnchorDateOfMargining = NaiveDateTime;

// impl TraitTermDescription for CycleAnchorDateOfMargining {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfMargining"
//     }
//     fn get_group(&self) -> &str {
//         "Margining"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Margining"
//     }
//     fn get_acronym(&self) -> &str {
//         "MRANX"
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
//         "Date from which the margin call date schedule is calculated according to the cycle length. The first margin call event takes place on this anchor."
//     }
// }    