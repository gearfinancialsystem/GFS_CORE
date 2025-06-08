use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::traits::DayCountConventionTrait::DayCountConventionTrait;

#[derive(PartialEq, Eq, Debug)]
pub struct A336;
impl A336 {
    pub fn new() -> Self {
        A336
    }
}

impl DayCountConventionTrait for A336 {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        (end_time - start_time).num_days() as f64
    }

    /// Calculates the day count fraction between two dates using the A/336 convention
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        Self::day_count(&self, start_time, end_time) as f64 / 336.0
    }
}