use chrono::NaiveDateTime;
use crate::traits::TraitTermDescription::TraitTermDescription;

pub type BoundaryMonitoringEndDate = NaiveDateTime;


impl TraitTermDescription for BoundaryMonitoringEndDate {
    fn get_identifier(&self) -> &str {
        "boundaryMonitoringEndDate"
    }
    fn get_group(&self) -> &str {
        "Barrier"
    }
    fn get_name(&self) -> &str {
        "Boundary Monitoring End Date"
    }
    fn get_acronym(&self) -> &str {
        "BMED"
    }
    fn get_type(&self) -> &str {
        "Timestamp"
    }
    fn get_allowed_values(&self) -> &str {
        "['ISO8601 Datetime']"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "Boundary monitoring ends on this date"
    }
}    