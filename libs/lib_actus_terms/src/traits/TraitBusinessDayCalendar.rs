use std::fmt::Debug;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

pub trait TraitBusinessDayCalendar: Debug{
    /// Returns whether a date is a business day or not
    ///
    /// # Arguments
    ///
    /// * `date` - The date to be checked
    ///
    /// # Returns
    ///
    /// * `true` if `date` is a business day, `false` otherwise
    fn is_business_day(&self, date: &IsoDatetime) -> bool;
}


