use chrono::NaiveDateTime;
use crate::traits::TraitCountConvention::TraitDayCountConvention;

use crate::traits::TraitDayCountConvention::TraitDayCountConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(PartialEq, Eq, Debug)]
pub struct A365;
impl A365 {
    pub fn new() -> Self {
        A365
    }
}

impl TraitDayCountConvention for A365 {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: NaiveDateTime, end_time: NaiveDateTime) -> f64 {
        (end_time - start_time).num_days() as f64
    }

    /// Calculates the day count fraction between two dates using the ISDA A/365-Fixed convention
    fn day_count_fraction(&self, start_time: NaiveDateTime, end_time: NaiveDateTime) -> f64 {
        Self::day_count(&self, start_time, end_time) as f64 / 365.0
    }
}

