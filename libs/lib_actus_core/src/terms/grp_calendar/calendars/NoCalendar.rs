
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

