use std::{fmt, ptr};
use std::rc::Rc;

use crate::types::IsoDatetime::IsoDatetime;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use chrono::Duration;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_interest::daycountconventions::AAISDA::AAISDA;
use crate::traits::TraitCountConvention::TraitDayCountConvention;

#[derive(Clone, Debug)]
pub struct B252 {
    pub calendar: Rc<Calendar>,
}

impl PartialEq for B252 {
    fn eq(&self, other: &Self) -> bool {
        // Compare l'adresse des trait objects contenus dans le Box.
        ptr::eq(self.calendar.as_ref(), other.calendar.as_ref())
    }
}

impl Eq for B252 {}

impl B252 {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        B252 {calendar}
    }
}



impl TraitDayCountConvention for B252 {

    /// Calculates the number of business days between two dates
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        let mut date = start_time;
        let mut days_count = 0;

        for _ in 0..(end_time - start_time).num_days() {
            if self.calendar.is_business_day(&date) {
                days_count += 1;
            }
            date = date + Duration::days(1);
        }

        days_count as f64
    }

    /// Calculates the day count fraction based on the Business-252 convention
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        self.day_count(start_time, end_time) as f64 / 252.0
    }
}

impl fmt::Display for B252 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "B252")
    }
}