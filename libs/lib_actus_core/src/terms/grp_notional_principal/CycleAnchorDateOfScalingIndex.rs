use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleAnchorDateOfScalingIndex = NaiveDateTime;

// impl TraitTermDescription for CycleAnchorDateOfScalingIndex {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfScalingIndex"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Scaling Index"
//     }
//     fn get_acronym(&self) -> &str {
//         "SCANX"
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
//         "Date from which the scaling date schedule is calculated according to the cycle length. The first scaling event takes place on this anchor."
//     }
// }    
