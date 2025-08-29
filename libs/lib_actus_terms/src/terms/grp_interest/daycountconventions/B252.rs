use std::{fmt, ptr};
use std::rc::Rc;

use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;
use crate::terms::grp_calendar::Calendar::Calendar;
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
    fn day_count(&self, start_time: PhantomIsoDatetimeW, end_time: PhantomIsoDatetimeW) -> f64 {
        let mut date = start_time;
        let mut days_count = 0;
        let q = end_time.numdays_between_dates(&start_time) as i32; // A voir
        for _ in 0..q {
            if self.calendar.is_business_day(&date) {
                days_count += 1;
            }
            date = date.add_period(PhantomIsoPeriodW::new(0,0,1));
                
        }

        days_count as f64
    }

    /// Calculates the day count fraction based on the Business-252 convention
    fn day_count_fraction(&self, start_time: PhantomIsoDatetimeW, end_time: PhantomIsoDatetimeW) -> f64 {
        self.day_count(start_time, end_time) as f64 / 252.0
    }
}

impl fmt::Display for B252 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "B252")
    }
}