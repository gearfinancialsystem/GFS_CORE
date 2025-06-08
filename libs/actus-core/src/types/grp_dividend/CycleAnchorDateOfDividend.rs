use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleAnchorDateOfDividend = IsoDatetime;

// impl TermDescriptionTrait for CycleAnchorDateOfDividend {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfDividend"
//     }
//     fn get_group(&self) -> &str {
//         "Dividend"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Dividend"
//     }
//     fn get_acronym(&self) -> &str {
//         "DVANX"
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
//         "Date from which the dividend payment date schedule is calculated according to the cycle length. The first dividend payment event takes place on this anchor."
//     }
// }    