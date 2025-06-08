use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TermDescriptionTrait::TermDescriptionTrait;

pub type CycleOfScalingIndex = IsoDuration;


// impl TermDescriptionTrait for CycleOfScalingIndex {
//     fn get_identifier(&self) -> &str {
//         "cycleOfScalingIndex"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Scaling Index"
//     }
//     fn get_acronym(&self) -> &str {
//         "SCCL"
//     }
//     fn get_type(&self) -> &str {
//         "Cycle"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['[ISO8601 Duration]L[s={0,1}]']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "Cycle according to which the scaling date schedule will be calculated.
// In case SCCL is not set, then there will only be one scaling event at SCANX given SCANX is set.
// The interval will be adjusted yet by EOMC and BDC."
//     }
// }    
