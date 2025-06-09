use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleAnchorDateOfInterestPayment = NaiveDateTime;

// impl TraitTermDescription for CycleAnchorDateOfInterestPayment {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfInterestPayment"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Interest Payment"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPANX"
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
//         "Date from which the interest payment date schedule is calculated according to the cycle length. The first interest payment event takes place on this anchor."
//     }
// }