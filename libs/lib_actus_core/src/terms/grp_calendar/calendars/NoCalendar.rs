
use chrono::NaiveDateTime;
use crate::terms::grp_calendar::calendars::MondayToFriday::MF;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

/// No holiday calendar
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct NC;

impl NC {
    pub fn new() -> Self {
        NC
    }
    pub fn type_str(&self) -> String {
        return "NC (No calendar) calendar".to_string();
    }
}

impl TraitBusinessDayCalendar for NC {
    fn is_business_day(&self, _date: &NaiveDateTime) -> bool {
        true
    }
}

impl TraitEnumOptionDescription for NC {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "noCalendar"
    }
    fn get_name(&self) -> &str {
        "No Calendar"
    }
    fn get_acronym(&self) -> &str {
        "NC"
    }

    fn get_description(&self) -> &str {
        "No holidays defined\r"
    }
}