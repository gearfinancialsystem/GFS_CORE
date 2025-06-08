use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleAnchorDateOfInterestCalculationBase = IsoDatetime;

// impl TermDescriptionTrait for CycleAnchorDateOfInterestCalculationBase {
//     fn get_identifier(&self) -> &str {
//         "cycleAnchorDateOfInterestCalculationBase"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Anchor Date Of Interest Calculation Base"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPCBANX"
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
//         "Date from which the interest calculation base date schedule is calculated according to the cycle length. The first interest calculation base event takes place on this anchor."
//     }
// }    
    

