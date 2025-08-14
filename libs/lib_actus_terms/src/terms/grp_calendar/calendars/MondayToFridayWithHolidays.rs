use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use std::{collections::HashSet, fmt};
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

/// Monday to Friday Calendar
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MFWH {
    pub holidays: HashSet<PhantomIsoDatetimeW>
};

impl MFWH {
    pub fn new() -> Self {
        MFWH
    }

}

impl TraitBusinessDayCalendar for MFWH {
    fn is_business_day(&self, date: &PhantomIsoDatetimeW) -> bool {
        let day_of_week = date.weekday().number_from_monday();
        day_of_week <= 5
    }
}



impl fmt::Display for MFWH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "MFWH")
    }
}
