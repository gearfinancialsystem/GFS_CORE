use crate::subtypes::IsoDuration::IsoDuration;


pub type BoundaryMonitoringCycle = IsoDuration;

// impl TermDescriptionTrait for BoundaryMonitoringCycle {
//     fn get_identifier(&self) -> &str {
//         "boundaryMonitoringCycle"
//     }
//     fn get_group(&self) -> &str {
//         "Boundary"
//     }
//     fn get_name(&self) -> &str {
//         "Boundary Monitoring Cycle"
//     }
//     fn get_acronym(&self) -> &str {
//         "BMCL"
//     }
//     fn get_type(&self) -> &str {
//         "Cycle"
//     }
//     fn get_allowed_values(&self) -> &str {
//         "['ISO8601 Duration']"
//     }
//     fn get_default_value(&self) -> &str {
//         ""
//     }
//     fn get_description(&self) -> &str {
//         "The frequency with which boundary monitoring events occur. It defines how often the system checks to test whether the  market value of the underlying asset has crossed the boundary in the specified direction triggerring  a boundary crossing event."
//     }
// }    