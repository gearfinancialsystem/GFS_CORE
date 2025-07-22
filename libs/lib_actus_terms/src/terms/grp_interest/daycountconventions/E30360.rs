use std::fmt;
use crate::types::IsoDatetime::IsoDatetime;
use chrono::Datelike;
use crate::traits::TraitCountConvention::TraitDayCountConvention;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct E30360;
impl E30360 {
    pub fn new() -> Self {
        E30360
    }
}

impl TraitDayCountConvention for E30360 {
    /// Calculates the number of days between two dates using the ISDA 30E/360 convention
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        let d1 = if start_time.day() == 31 { 30.0 } else { start_time.day() as f64 };
        let d2 = if end_time.day() == 31 { 30.0 } else { end_time.day() as f64 };

        let del_d = d2 - d1;
        let del_m = end_time.month() as i32 - start_time.month() as i32;
        let del_y = end_time.year() - start_time.year();

        (360.0 * del_y as f64) + (30.0 * del_m as f64) + del_d
    }

    /// Calculates the day count fraction based on the ISDA 30E/360 convention
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        Self::day_count(&self, start_time, end_time) / 360.0
    }
}
impl fmt::Display for E30360 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E30360")
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
    fn test_daycount_thirty_e_three_sixty_1() {
        let start1 = parse_date("2006-01-31T00:00:00");
        let end1 = parse_date("2006-02-28T00:00:00");
        let result = 28.0;
        assert_eq!(result, E30360.day_count(IsoDatetime(start1), IsoDatetime(end1)) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_2() {
        let start2 = parse_date("2006-01-30T00:00:00");
        let end2 = parse_date("2006-02-28T00:00:00");
        let result = 28.0;
        assert_eq!(result, E30360.day_count(IsoDatetime(start2), IsoDatetime(end2)) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_3() {
        let start3 = parse_date("2006-02-28T00:00:00");
        let end3 = parse_date("2006-03-03T00:00:00");
        let result = 5.0;
        assert_eq!(result, E30360.day_count(IsoDatetime(start3), IsoDatetime(end3)) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_4() {
        let start4 = parse_date("2006-02-14T00:00:00");
        let end4 = parse_date("2006-02-28T00:00:00");
        let result = 14.0;
        assert_eq!(result, E30360.day_count(IsoDatetime(start4), IsoDatetime(end4)) as f64);
    }

    // #[test]
    // fn test_daycount_thirty_e_three_sixty_5() {
    //     let start5 = parse_date("2006-09-30T00:00:00");
    //     let end5 = parse_date("2006-10-31T00:00:00");
    //     let result = 30.0;
    //     assert_eq!(result, E30360.day_count(start5, end5) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_6() {
    //     let start6 = parse_date("2006-10-31T00:00:00");
    //     let end6 = parse_date("2006-11-28T00:00:00");
    //     let result = 28.0;
    //     assert_eq!(result, E30360.day_count(start6, end6) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_7() {
    //     let start7 = parse_date("2007-08-31T00:00:00");
    //     let end7 = parse_date("2008-02-28T00:00:00");
    //     let result = 178.0;
    //     assert_eq!(result, E30360.day_count(start7, end7) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_8() {
    //     let start8 = parse_date("2008-02-28T00:00:00");
    //     let end8 = parse_date("2008-08-28T00:00:00");
    //     let result = 180.0;
    //     assert_eq!(result, E30360.day_count(start8, end8) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_9() {
    //     let start9 = parse_date("2008-02-28T00:00:00");
    //     let end9 = parse_date("2008-08-30T00:00:00");
    //     let result = 182.0;
    //     assert_eq!(result, E30360.day_count(start9, end9) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_10() {
    //     let start10 = parse_date("2008-02-28T00:00:00");
    //     let end10 = parse_date("2008-08-31T00:00:00");
    //     let result = 182.0;
    //     assert_eq!(result, E30360.day_count(start10, end10) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_11() {
    //     let start11 = parse_date("2007-02-26T00:00:00");
    //     let end11 = parse_date("2008-02-28T00:00:00");
    //     let result = 362.0;
    //     assert_eq!(result, E30360.day_count(start11, end11) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_12() {
    //     let start12 = parse_date("2007-02-26T00:00:00");
    //     let end12 = parse_date("2008-02-29T00:00:00");
    //     let result = 363.0;
    //     assert_eq!(result, E30360.day_count(start12, end12) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_13() {
    //     let start13 = parse_date("2008-02-29T00:00:00");
    //     let end13 = parse_date("2009-02-28T00:00:00");
    //     let result = 359.0;
    //     assert_eq!(result, E30360.day_count(start13, end13) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_14() {
    //     let start14 = parse_date("2008-02-28T00:00:00");
    //     let end14 = parse_date("2008-03-30T00:00:00");
    //     let result = 32.0;
    //     assert_eq!(result, E30360.day_count(start14, end14) as f64);
    // }
    // 
    // #[test]
    // fn test_daycount_thirty_e_three_sixty_15() {
    //     let start15 = parse_date("2008-02-28T00:00:00");
    //     let end15 = parse_date("2008-03-31T00:00:00");
    //     let result = 32.0;
    //     assert_eq!(result, E30360.day_count(start15, end15) as f64);
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_1() {
    //     let start1 = parse_date("2006-01-31T00:00:00");
    //     let end1 = parse_date("2006-02-28T00:00:00");
    //     let result = 28.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start1, end1));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_2() {
    //     let start2 = parse_date("2006-01-30T00:00:00");
    //     let end2 = parse_date("2006-02-28T00:00:00");
    //     let result = 28.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start2, end2));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_3() {
    //     let start3 = parse_date("2006-02-28T00:00:00");
    //     let end3 = parse_date("2006-03-03T00:00:00");
    //     let result = 5.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start3, end3));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_4() {
    //     let start4 = parse_date("2006-02-14T00:00:00");
    //     let end4 = parse_date("2006-02-28T00:00:00");
    //     let result = 14.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start4, end4));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_5() {
    //     let start5 = parse_date("2006-09-30T00:00:00");
    //     let end5 = parse_date("2006-10-31T00:00:00");
    //     let result = 30.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start5, end5));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_6() {
    //     let start6 = parse_date("2006-10-31T00:00:00");
    //     let end6 = parse_date("2006-11-28T00:00:00");
    //     let result = 28.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start6, end6));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_7() {
    //     let start7 = parse_date("2007-08-31T00:00:00");
    //     let end7 = parse_date("2008-02-28T00:00:00");
    //     let result = 178.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start7, end7));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_8() {
    //     let start8 = parse_date("2008-02-28T00:00:00");
    //     let end8 = parse_date("2008-08-28T00:00:00");
    //     let result = 180.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start8, end8));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_9() {
    //     let start9 = parse_date("2008-02-28T00:00:00");
    //     let end9 = parse_date("2008-08-30T00:00:00");
    //     let result = 182.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start9, end9));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_10() {
    //     let start10 = parse_date("2008-02-28T00:00:00");
    //     let end10 = parse_date("2008-08-31T00:00:00");
    //     let result = 182.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start10, end10));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_11() {
    //     let start11 = parse_date("2007-02-26T00:00:00");
    //     let end11 = parse_date("2008-02-28T00:00:00");
    //     let result = 362.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start11, end11));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_12() {
    //     let start12 = parse_date("2007-02-26T00:00:00");
    //     let end12 = parse_date("2008-02-29T00:00:00");
    //     let result = 363.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start12, end12));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_13() {
    //     let start13 = parse_date("2008-02-29T00:00:00");
    //     let end13 = parse_date("2009-02-28T00:00:00");
    //     let result = 359.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start13, end13));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_14() {
    //     let start14 = parse_date("2008-02-28T00:00:00");
    //     let end14 = parse_date("2008-03-30T00:00:00");
    //     let result = 32.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start14, end14));
    // }
    // 
    // #[test]
    // fn test_fraction_thirty_e_three_sixty_15() {
    //     let start15 = parse_date("2008-02-28T00:00:00");
    //     let end15 = parse_date("2008-03-31T00:00:00");
    //     let result = 32.0 / 360.0;
    //     assert_eq!(result, E30360.day_count_fraction(start15, end15));
    // }
}