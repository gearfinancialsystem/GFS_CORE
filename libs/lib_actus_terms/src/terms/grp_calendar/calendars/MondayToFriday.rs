use chrono::Datelike;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use std::fmt;

/// Monday to Friday Calendar
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MF;

impl MF {
    pub fn new() -> Self {
        MF
    }

}

impl TraitBusinessDayCalendar for MF {
    fn is_business_day(&self, date: &IsoDatetime) -> bool {
        let day_of_week = date.weekday().number_from_monday();
        day_of_week <= 5
    }
}



impl fmt::Display for MF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "MF")
    }
}
