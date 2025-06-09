use chrono::NaiveDateTime;

pub type BoundaryMonitoringAnchorDate = NaiveDateTime;


// impl TraitTermDescription for BoundaryMonitoringAnchorDate {
//     fn get_identifier(&self) -> &str {
//         "boundaryMonitoringAnchorDate"
//     }
//     fn get_group(&self) -> &str {
//         "Boundary"
//     }
//     fn get_name(&self) -> &str {
//         "Boundary Monitoring Anchor Date"
//     }
//     fn get_acronym(&self) -> &str {
//         "BMANX"
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
//         "The first Boundary monitoring event occurs on this date"
//     }
// }    