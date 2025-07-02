use std::fmt;
use crate::types::IsoDatetime::IsoDatetime;
use chrono::{Datelike, NaiveDate};
use crate::terms::grp_interest::daycountconventions::A365::A365;
use crate::traits::TraitCountConvention::TraitDayCountConvention;

#[derive(Clone, PartialEq, Eq, Debug)]
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

impl fmt::Display for AAISDA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AAISDA")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::AAISDA;
    use chrono::NaiveDateTime;

    fn parse_date(date_str: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date")
    }

    #[test]
    fn test_daycount_actual_actual_isda_1() {
        let local_date1 = parse_date("2019-02-01T00:00:00");
        let local_date2 = parse_date("2019-03-30T00:00:00");
        let result = 57.0;
        assert_eq!(result, AAISDA.day_count(local_date1, local_date2));
    }

    #[test]
    fn test_day_count_fraction_actual_actual_isda_1() {
        let local_date1 = parse_date("2019-02-01T00:00:00");
        let local_date2 = parse_date("2019-03-30T00:00:00");
        let result = 0.15616438356164383; // 57 divided by 365 (not leap year basis)
        assert_eq!(result, AAISDA.day_count_fraction(local_date1, local_date2));
    }

    #[test]
    fn test_daycount_actual_actual_isda_2() {
        let local_date1 = parse_date("2019-02-01T00:00:00");
        let local_date3 = parse_date("2019-07-30T00:00:00");
        let result = 179.0;
        assert_eq!(result, AAISDA.day_count(local_date1, local_date3) as f64);
    }

    #[test]
    fn test_day_count_fraction_actual_actual_isda_2() {
        let local_date1 = parse_date("2019-02-01T00:00:00");
        let local_date3 = parse_date("2019-07-30T00:00:00");
        let result = 0.4904109589041096; // 179 divided by 365 (not leap year basis)
        assert_eq!(result, AAISDA.day_count_fraction(local_date1, local_date3));
    }
}
