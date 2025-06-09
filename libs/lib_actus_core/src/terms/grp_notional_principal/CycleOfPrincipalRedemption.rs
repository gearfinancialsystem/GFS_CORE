use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type CycleOfPrincipalRedemption = IsoDuration;

// impl TraitTermDescription for CycleOfPrincipalRedemption {
//     fn get_identifier(&self) -> &str {
//         "cycleOfPrincipalRedemption"
//     }
//     fn get_group(&self) -> &str {
//         "Notional Principal"
//     }
//     fn get_name(&self) -> &str {
//         "Cycle Of Principal Redemption"
//     }
//     fn get_acronym(&self) -> &str {
//         "PRCL"
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
// In case PRCL is not set, then there will only be one principal payment event at MD (and possibly at PRANX if set).
// The interval will be adjusted yet by EOMC and BDC."
//     }
// }    