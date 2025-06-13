
use std::ptr;
use chrono::NaiveDateTime;
use chrono::Duration;
use std::rc::Rc;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;

/// Implementation of the Preceding business day convention
///
/// This business day convention assumes that if a date falls on a non-business day,
/// it is shifted to the previous business day. Hence, if `d` is the initial date
/// and `d'` is the shifted date, we have that:
/// - `d' = d` if `d` is a business day
/// - `d' < d` if `d` is a non-business day


#[derive(Clone, Debug)]
pub struct Preceeding {
    pub calendar: Rc<dyn TraitBusinessDayCalendar>,
}

impl PartialEq for Preceeding {
    fn eq(&self, other: &Self) -> bool {
        // Compare l'adresse des trait objects contenus dans le Box.
        ptr::eq(self.calendar.as_ref(), other.calendar.as_ref())
    }
}

impl Eq for Preceeding {}

impl Preceeding {
    pub fn new(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        Preceeding { calendar }
    }
}

impl TraitBusinessDayConvention for Preceeding {
    /// Shift the input date to the closest business day if it is a non-business day
    ///
    /// # Arguments
    ///
    /// * `date` - The date to be shifted
    ///
    /// # Returns
    ///
    /// * The shifted date (a business day)
    fn shift(&self, date: &NaiveDateTime) -> NaiveDateTime {
        let mut shifted_date = *date;
        // Move backward to the previous business day
        while !self.calendar.is_business_day(&shifted_date) {
            shifted_date -= Duration::days(1);
        }
        shifted_date
    }
}
