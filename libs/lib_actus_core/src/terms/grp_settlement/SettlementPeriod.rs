use crate::subtypes::IsoDuration::IsoDuration;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type SettlementPeriod = IsoDuration;

// impl TraitTermDescription for SettlementPeriod {
//     fn get_identifier(&self) -> &str {
//         "settlementPeriod"
//     }
//     fn get_group(&self) -> &str {
//         "Settlement"
//     }
//     fn get_name(&self) -> &str {
//         "Settlement Period"
//     }
//     fn get_acronym(&self) -> &str {
//         "STP"
//     }
//     fn get_type(&self) -> &str {
//         "Period"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Duration']"
//     }
//     fn get_default_value(&self) -> &str {
//         "P0D"
//     }
//     fn get_description(&self) -> &str {
//         "Defines the period from fixing of a contingent event/obligation (Exercise Date) to settlement of the obligation.
// "
//     }
// }    
    