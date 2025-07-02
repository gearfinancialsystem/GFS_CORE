use std::fmt;
use crate::terms::grp_interest::daycountconventions::A360::A360;
use crate::types::IsoDatetime::IsoDatetime;
use crate::traits::TraitCountConvention::TraitDayCountConvention;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct A365;
impl A365 {
    pub fn new() -> Self {
        A365
    }
}

impl TraitDayCountConvention for A365 {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        (end_time - start_time).num_days() as f64
    }

    /// Calculates the day count fraction between two dates using the ISDA A/365-Fixed convention
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        Self::day_count(&self, start_time, end_time) as f64 / 365.0
    }
}
impl fmt::Display for A365 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A365")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn parse_date(date_str: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date")
    }

    #[test]
    fn test_daycount_actual_three_sixty_five_fixed_1() {
        let start1 = parse_date("2006-01-31T00:00:00");
        let end1 = parse_date("2006-02-28T00:00:00");
        let result = 28.0;
        assert_eq!(result, A365.day_count(start1, end1));
    }

    #[test]
    fn test_daycount_actual_three_sixty_five_fixed_2() {
        let start2 = parse_date("2006-01-30T00:00:00");
        let end2 = parse_date("2006-02-28T00:00:00");
        let result = 29.0;
        assert_eq!(result, A365.day_count(start2, end2));
    }

    #[test]
    fn test_daycount_actual_three_sixty_five_fixed_3() {
        let start3 = parse_date("2006-02-28T00:00:00");
        let end3 = parse_date("2006-03-03T00:00:00");
        let result = 3.0;
        assert_eq!(result, A365.day_count(start3, end3) as f64);
    }

    #[test]
    fn test_daycount_actual_three_sixty_five_fixed_4() {
        let start4 = parse_date("2006-02-14T00:00:00");
        let end4 = parse_date("2006-02-28T00:00:00");
        let result = 14.0;
        assert_eq!(result, A365.day_count(start4, end4) as f64);
    }

    #[test]
    fn test_daycount_actual_three_sixty_five_fixed_5() {
        let start5 = parse_date("2006-09-30T00:00:00");
        let end5 = parse_date("2006-10-31T00:00:00");
        let result = 31.0;
        assert_eq!(result, A365.day_count(start5, end5) as f64);
    }

    #[test]
    fn test_fraction_actual_three_sixty_five_fixed_1() {
        let start1 = parse_date("2006-01-31T00:00:00");
        let end1 = parse_date("2006-02-28T00:00:00");
        let result = 28.0 / 365.0;
        assert_eq!(result, A365.day_count_fraction(start1, end1));
    }

    #[test]
    fn test_fraction_actual_three_sixty_five_fixed_2() {
        let start2 = parse_date("2006-01-30T00:00:00");
        let end2 = parse_date("2006-02-28T00:00:00");
        let result = 29.0 / 365.0;
        assert_eq!(result, A365.day_count_fraction(start2, end2));
    }

    #[test]
    fn test_fraction_actual_three_sixty_five_fixed_3() {
        let start3 = parse_date("2006-02-28T00:00:00");
        let end3 = parse_date("2006-03-03T00:00:00");
        let result = 3.0 / 365.0;
        assert_eq!(result, A365.day_count_fraction(start3, end3));
    }

    #[test]
    fn test_fraction_actual_three_sixty_five_fixed_4() {
        let start4 = parse_date("2006-02-14T00:00:00");
        let end4 = parse_date("2006-02-28T00:00:00");
        let result = 14.0 / 365.0;
        assert_eq!(result, A365.day_count_fraction(start4, end4));
    }

    #[test]
    fn test_fraction_actual_three_sixty_five_fixed_5() {
        let start5 = parse_date("2006-09-30T00:00:00");
        let end5 = parse_date("2006-10-31T00:00:00");
        let result = 31.0 / 365.0;
        assert_eq!(result, A365.day_count_fraction(start5, end5));
    }
}
