
use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::calendars::MondayToFriday::MF;


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

impl BusinessDayCalendarTrait for NC {
    fn is_business_day(&self, _date: &IsoDatetime) -> bool {
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