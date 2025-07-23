use chrono::NaiveDateTime;

use crate::types::IsoDatetime::IsoDatetime;
pub trait TraitDayCountConvention {
    /// Compute the number of days between two time instances
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64;

    /// Compute the number of days as a fraction of total number of days in the reference year
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64;
}
