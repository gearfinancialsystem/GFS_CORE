use std::fmt;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::traits::TraitCountConvention::TraitDayCountConvention;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct A360;
impl A360 {
    pub fn new() -> Self {
        A360
    }
}

impl TraitDayCountConvention for A360 {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: PhantomIsoDatetimeW, end_time: PhantomIsoDatetimeW) -> f64 {
        end_time.numdays_between_dates(&start_time)
    }

    /// Calculates the day count fraction between two dates using the ISDA A/360 convention
    fn day_count_fraction(&self, start_time: PhantomIsoDatetimeW, end_time: PhantomIsoDatetimeW) -> f64 {
        Self::day_count(&self, start_time, end_time) as f64 / 360.0
    }
}
impl fmt::Display for A360 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A360")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::NaiveDateTime;
// 
//     fn parse_date(date_str: &str) -> NaiveDateTime {
//         NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date")
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_1() {
//         let start1 = IsoDatetime(parse_date("2006-01-31T00:00:00"));
//         let end1 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let result = 28.0;
//         assert_eq!(result, A360.day_count(start1, end1) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_2() {
//         let start2 = IsoDatetime(parse_date("2006-01-30T00:00:00"));
//         let end2 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let result = 29.0;
//         assert_eq!(result, A360.day_count(start2, end2) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_3() {
//         let start3 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let end3 = IsoDatetime(parse_date("2006-03-03T00:00:00"));
//         let result = 3.0;
//         assert_eq!(result, A360.day_count(start3, end3) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_4() {
//         let start4 = IsoDatetime(parse_date("2006-02-14T00:00:00"));
//         let end4 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let result = 14.0;
//         assert_eq!(result, A360.day_count(start4, end4) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_5() {
//         let start5 = IsoDatetime(parse_date("2006-09-30T00:00:00"));
//         let end5 = IsoDatetime(parse_date("2006-10-31T00:00:00"));
//         let result = 31.0;
//         assert_eq!(result, A360.day_count(start5, end5) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_6() {
//         let start6 = IsoDatetime(parse_date("2006-10-31T00:00:00"));
//         let end6 = IsoDatetime(parse_date("2006-11-28T00:00:00"));
//         let result = 28.0;
//         assert_eq!(result, A360.day_count(start6, end6) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_7() {
//         let start7 = IsoDatetime(parse_date("2007-08-31T00:00:00"));
//         let end7 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let result = 181.0;
//         assert_eq!(result, A360.day_count(start7, end7) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_8() {
//         let start8 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end8 = IsoDatetime(parse_date("2008-08-28T00:00:00"));
//         let result = 182.0;
//         assert_eq!(result, A360.day_count(start8, end8) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_9() {
//         let start9 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end9 = IsoDatetime(parse_date("2008-08-30T00:00:00"));
//         let result = 184.0;
//         assert_eq!(result, A360.day_count(start9, end9) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_10() {
//         let start10 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end10 = IsoDatetime(parse_date("2008-08-31T00:00:00"));
//         let result = 185.0;
//         assert_eq!(result, A360.day_count(start10, end10) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_11() {
//         let start11 = IsoDatetime(parse_date("2007-02-26T00:00:00"));
//         let end11 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let result = 367.0;
//         assert_eq!(result, A360.day_count(start11, end11) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_12() {
//         let start12 = IsoDatetime(parse_date("2007-02-26T00:00:00"));
//         let end12 = IsoDatetime(parse_date("2008-02-29T00:00:00"));
//         let result = 368.0;
//         assert_eq!(result, A360.day_count(start12, end12) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_13() {
//         let start13 = IsoDatetime(parse_date("2008-02-29T00:00:00"));
//         let end13 = IsoDatetime(parse_date("2009-02-28T00:00:00"));
//         let result = 365.0;
//         assert_eq!(result, A360.day_count(start13, end13) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_14() {
//         let start14 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end14 = IsoDatetime(parse_date("2008-03-30T00:00:00"));
//         let result = 31.0;
//         assert_eq!(result, A360.day_count(start14, end14) as f64);
//     }
// 
//     #[test]
//     fn test_daycount_actual_three_sixty_15() {
//         let start15 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end15 = IsoDatetime(parse_date("2008-03-31T00:00:00"));
//         let result = 32.0;
//         assert_eq!(result, A360.day_count(start15, end15) as f64);
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_1() {
//         let start1 = IsoDatetime(parse_date("2006-01-31T00:00:00"));
//         let end1 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let result = 28.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start1, end1));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_2() {
//         let start2 = IsoDatetime(parse_date("2006-01-30T00:00:00"));
//         let end2 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let result = 29.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start2, end2));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_3() {
//         let start3 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let end3 = IsoDatetime(parse_date("2006-03-03T00:00:00"));
//         let result = 3.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start3, end3));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_4() {
//         let start4 = IsoDatetime(parse_date("2006-02-14T00:00:00"));
//         let end4 = IsoDatetime(parse_date("2006-02-28T00:00:00"));
//         let result = 14.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start4, end4));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_5() {
//         let start5 = IsoDatetime(parse_date("2006-09-30T00:00:00"));
//         let end5 = IsoDatetime(parse_date("2006-10-31T00:00:00"));
//         let result = 31.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start5, end5));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_6() {
//         let start6 = IsoDatetime(parse_date("2006-10-31T00:00:00"));
//         let end6 = IsoDatetime(parse_date("2006-11-28T00:00:00"));
//         let result = 28.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start6, end6));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_7() {
//         let start7 = IsoDatetime(parse_date("2007-08-31T00:00:00"));
//         let end7 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let result = 181.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start7, end7));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_8() {
//         let start8 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end8 = IsoDatetime(parse_date("2008-08-28T00:00:00"));
//         let result = 182.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start8, end8));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_9() {
//         let start9 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end9 = IsoDatetime(parse_date("2008-08-30T00:00:00"));
//         let result = 184.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start9, end9));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_10() {
//         let start10 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end10 = IsoDatetime(parse_date("2008-08-31T00:00:00"));
//         let result = 185.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start10, end10));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_11() {
//         let start11 = IsoDatetime(parse_date("2007-02-26T00:00:00"));
//         let end11 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let result = 367.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start11, end11));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_12() {
//         let start12 = IsoDatetime(parse_date("2007-02-26T00:00:00"));
//         let end12 = IsoDatetime(parse_date("2008-02-29T00:00:00"));
//         let result = 368.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start12, end12));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_13() {
//         let start13 = IsoDatetime(parse_date("2008-02-29T00:00:00"));
//         let end13 = IsoDatetime(parse_date("2009-02-28T00:00:00"));
//         let result = 365.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start13, end13));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_14() {
//         let start14 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end14 = IsoDatetime(parse_date("2008-03-30T00:00:00"));
//         let result = 31.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start14, end14));
//     }
// 
//     #[test]
//     fn test_fraction_actual_three_sixty_15() {
//         let start15 = IsoDatetime(parse_date("2008-02-28T00:00:00"));
//         let end15 = IsoDatetime(parse_date("2008-03-31T00:00:00"));
//         let result = 32.0 / 360.0;
//         assert_eq!(result, A360.day_count_fraction(start15, end15));
//     }
// }

