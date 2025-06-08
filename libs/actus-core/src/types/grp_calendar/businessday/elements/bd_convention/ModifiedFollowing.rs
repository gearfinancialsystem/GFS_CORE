
use crate::subtypes::IsoDatetime::IsoDatetime;

use crate::terms::grp_calendar::Calendar::Calendar;
use chrono::{Datelike, Duration};
use std::ptr;
use std::rc::Rc;

/// Implementation of the Modified Following business day convention
///
/// This business day convention assumes that if a date falls on a non-business day,
/// it is shifted to the next following business day if this is in the same month,
/// or to the preceding business day otherwise. Hence, if `d` is the initial date
/// and `d'` is the shifted date, we have that:
/// - `d' = d` if `d` is a business day
/// - `d' > d` if `d` is a non-business day and the next following business day is in the same month as `d`
/// - `d' < d` if `d` is a non-business day and the next following business day is not in the same month as `d`


#[derive(Debug)]
pub struct ModifiedFollowing {
    pub calendar: Rc<dyn BusinessDayCalendarTrait>,
}

impl PartialEq for ModifiedFollowing {
    fn eq(&self, other: &Self) -> bool {
        // Compare l'adresse des trait objects contenus dans le Box.
        ptr::eq(self.calendar.as_ref(), other.calendar.as_ref())
    }
}

impl Eq for ModifiedFollowing {}

impl ModifiedFollowing {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `calendar` - The `BusinessDayCalendarProvider` to be used
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        ModifiedFollowing { calendar }
    }
}



impl BusinessDayConventionTrait for ModifiedFollowing {
    /// Shift the input date to the closest business day according to the Modified Following convention
    ///
    /// # Arguments
    ///
    /// * `date` - The date to be shifted
    ///
    /// # Returns
    ///
    /// * The shifted date (a business day)
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime {
        let mut shifted_date = *date;
        while !self.calendar.is_business_day(&shifted_date) {
            shifted_date += Duration::days(1);
        }
        if shifted_date.month() != date.month() {
            shifted_date = *date;
            while !self.calendar.is_business_day(&shifted_date) {
                shifted_date -= Duration::days(1);
            }
        }
        shifted_date
    }
}