
use chrono::NaiveDateTime;
use chrono::Datelike;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

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
    fn is_business_day(&self, date: &NaiveDateTime) -> bool {
        let day_of_week = date.weekday().number_from_monday();
        day_of_week <= 5
    }
}

impl TraitEnumOptionDescription for MF {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "mondayToFriday"
    }
    fn get_name(&self) -> &str {
        "mondayToFriday"
    }
    fn get_acronym(&self) -> &str {
        "MF"
    }

    fn get_description(&self) -> &str {
        "Saturdays and Sundays are holidays\r"
    }
}
