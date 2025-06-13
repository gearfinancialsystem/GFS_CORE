
use crate::types::isoDatetime::IsoDatetime;
use chrono::{Datelike, NaiveDate};
use crate::traits::TraitCountConvention::TraitDayCountConvention;

#[derive(PartialEq, Eq, Debug)]
pub struct AAISDA;

impl AAISDA {
    pub fn new() -> Self {
        AAISDA
    }
}
impl TraitDayCountConvention for AAISDA {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        (end_time - start_time).num_days() as f64
    }

    /// Calculates the day count fraction between two dates using the ISDA A/A convention
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        let y1 = start_time.year();
        let y2 = end_time.year();

        if y1 == y2 {
            let basis = if start_time.date().leap_year() { 366.0 } else { 365.0 };
            return (Self::day_count(&self, start_time, end_time) as f64) / basis;
        }

        let first_basis = if start_time.date().leap_year() { 366.0 } else { 365.0 };
        let second_basis = if end_time.date().leap_year() { 366.0 } else { 365.0 };

        // On suppose from_ymd_opt(...) => Some(NaiveDateTime).
        let days_in_first_year = Self::day_count(
            &self,
            start_time,
            NaiveDate::from_ymd_opt(y1 + 1, 1, 1).unwrap().and_hms_opt(1,1,1).unwrap()
        );
        let days_in_second_year = Self::day_count(
            &self,
            NaiveDate::from_ymd_opt(y2, 1, 1).unwrap().and_hms_opt(1,1,1).unwrap(),
            end_time
        );

        (days_in_first_year as f64 / first_basis)
            + (days_in_second_year as f64 / second_basis)
            + (y2 - y1 - 1) as f64
    }
}

