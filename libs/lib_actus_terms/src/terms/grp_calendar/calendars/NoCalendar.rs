use std::fmt;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

/// No holiday calendar
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct NC;

impl NC {
    pub fn new() -> Self {
        NC
    }

}

impl TraitBusinessDayCalendar for NC {
    fn is_business_day(&self, _date: &PhantomIsoDatetimeW) -> bool {
        true
    }
}


impl fmt::Display for NC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "NC")
    }
}


#[cfg(test)]
mod tests_calendars_NoCalendar {
    use lib_actus_types::types::IsoDatetime::IsoDatetime;
    use super::*;
    #[test]
    fn test_SAME_NoHolidaysCalendar() {
        let adjuster = "";
        
    }
}
