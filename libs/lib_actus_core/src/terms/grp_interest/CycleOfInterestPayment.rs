use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleOfInterestPayment = IsoDuration;

// impl TraitTermDescription for CycleOfInterestPayment {
//     fn get_identifier(&self) -> &str {
//         "cycleOfInterestPayment"
//     }
//     fn get_group(&self) -> &str {
//         "Interest"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Interest Payment"
//     }
//     fn get_acronym(&self) -> &str {
//         "IPCL"
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
//         "Cycle according to which the interest payment date schedule will be calculated.
// In case IPCL is not set, then there will only be an interest payment event at MD (and possibly at IPANX if set).
// The interval will be adjusted yet by EOMC and BDC."
//     }
// }