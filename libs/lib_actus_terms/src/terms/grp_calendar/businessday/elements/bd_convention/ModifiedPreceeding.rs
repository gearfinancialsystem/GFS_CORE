
use chrono::{Datelike, Duration};
use std::{fmt, ptr};
use std::rc::Rc;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_types::types::IsoPeriod::IsoPeriod;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;

/// Implementation of the Modified Preceding business day convention
///
/// This business day convention assumes that if a date falls on a non-business day,
/// it is shifted to the next preceding business day if this is in the same month,
/// or to the next following business day otherwise. Hence, if `d` is the initial date
/// and `d'` is the shifted date, we have that:
/// - `d' = d` if `d` is a business day
/// - `d' < d` if `d` is a non-business day and the next preceding business day is in the same month as `d`
/// - `d' > d` if `d` is a non-business day and the next preceding business day is not in the same month as `d`

#[derive(Clone, Debug)]
pub struct ModifiedPreceeding {
    pub calendar: Rc<Calendar>,
}

impl PartialEq for ModifiedPreceeding {
    fn eq(&self, other: &Self) -> bool {
        // Compare l'adresse des trait objects contenus dans le Box.
        ptr::eq(self.calendar.as_ref(), other.calendar.as_ref())
    }
}

impl Eq for ModifiedPreceeding {}

impl ModifiedPreceeding {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        ModifiedPreceeding { calendar }
    }
}

impl TraitBusinessDayAdjuster for ModifiedPreceeding {
    /// Shift the input date to the closest business day if it is a non-business day
    ///
    /// # Arguments
    ///
    /// * `date` - The date to be shifted
    ///
    /// # Returns
    ///
    /// * The shifted date (a business day)
    fn shift(&self, date: &PhantomIsoDatetimeW) -> PhantomIsoDatetimeW {
        let mut shifted_date = *date;
        while !self.calendar.is_business_day(&shifted_date) {
            // shifted_date -= Duration::days(1);
            shifted_date = shifted_date.sub_period(PhantomIsoPeriodW::new(0,0, 1));
        }
        if shifted_date.month() != date.month() {
            shifted_date = *date;
            while !self.calendar.is_business_day(&shifted_date) {
                //shifted_date += Duration::days(1);
                shifted_date = shifted_date.add_period(PhantomIsoPeriodW::new(0,0, 1));
            }
        }
        shifted_date
    }
}
impl fmt::Display for ModifiedPreceeding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Modified Preceeding (Calendar : {})", self.calendar.to_string())
    }
}