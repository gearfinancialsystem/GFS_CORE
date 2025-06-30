use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::types::IsoDatetime::IsoDatetime;

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
    fn is_business_day(&self, _date: &IsoDatetime) -> bool {
        true
    }
}

#[cfg(test)]
mod tests_calendars_NoCalendar {
    use crate::types::IsoDatetime::IsoDatetime;
    use super::*;
    #[test]
    fn test_SAME_NoHolidaysCalendar() {
        let adjuster = "";
        
    }
}
