use chrono::Datelike;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::types::isoDatetime::IsoDatetime;

/// Monday to Friday Calendar
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MF;

impl MF {
    pub fn new() -> Self {
        return MF;
    }
    pub fn type_str(&self) -> String {
        return "MF (Monday to Friday) calendar".to_string();
    }
}

impl TraitBusinessDayCalendar for MF {
    fn is_business_day(&self, date: &IsoDatetime) -> bool {
        let day_of_week = date.weekday().number_from_monday();
        day_of_week <= 5
    }
}


