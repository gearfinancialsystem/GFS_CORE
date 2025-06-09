use chrono::NaiveDateTime;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitDayCountConvention::TraitDayCountConvention;
use chrono::{Datelike, Duration};
use crate::traits::TraitCountConvention::TraitDayCountConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(PartialEq, Eq, Debug)]
pub struct A360;
impl A360 {
    pub fn new() -> Self {
        A360
    }
}

impl TraitDayCountConvention for A360 {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: NaiveDateTime, end_time: NaiveDateTime) -> f64 {
        (end_time - start_time).num_days() as f64
    }

    /// Calculates the day count fraction between two dates using the ISDA A/360 convention
    fn day_count_fraction(&self, start_time: NaiveDateTime, end_time: NaiveDateTime) -> f64 {
        Self::day_count(&self, start_time, end_time) as f64 / 360.0
    }
}

impl TraitEnumOptionDescription for A360 {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "actualThreeSixty"
    }
    fn get_name(&self) -> &str {
        "Actual Three Sixty"
    }
    fn get_acronym(&self) -> &str {
        "A360"
    }
    fn get_description(&self) -> &str {
        "Year fractions accrue on the basis of the actual number of days per month and 360 days per year in the respective period."
    }
}