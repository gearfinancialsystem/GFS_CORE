use std::fmt;
use std::rc::Rc;

use crate::types::IsoDatetime::IsoDatetime;
use crate::types::IsoDatetime::TraitNaiveDateTimeExtension;
use chrono::Datelike;
use crate::terms::grp_interest::daycountconventions::E30360ISDA::E30360ISDA;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::traits::TraitCountConvention::TraitDayCountConvention;


#[derive(Clone, PartialEq, Debug)]
pub struct E283666 {
    pub maturity_date: Option<Rc<MaturityDate>>,
}

impl E283666 {
    pub fn new(maturity_date: Option<Rc<MaturityDate>>) -> Self {
        E283666 {maturity_date}
    }
}
impl TraitDayCountConvention for E283666 {
    /// Calcule le nombre de jours, selon la convention 28/336
    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        // Ajustement de d1
        let mut d1 = start_time.day();
        if start_time.is_last_day_of_month() {
            d1 = 28;
        }

        // Ajustement de d2
        let mut d2 = end_time.day();

        if self.maturity_date.is_some() {
            let a = self.maturity_date.clone().map(|rc| (*rc).clone()).unwrap();
            if !(end_time == a.value() || end_time.month() == 2)
            && end_time.is_last_day_of_month() {
                d2 = 28;
            }
        }
        else if d2 > 28 {
            d2 = 28;
        }

        let del_d = d2 as f64 - d1 as f64;
        let del_m = end_time.month() as i32 - start_time.month() as i32;
        let del_y = end_time.year() - start_time.year();

        (336.0 * del_y as f64) + (28.0 * del_m as f64) + del_d
    }

    /// Calcule la fraction (days / 336.0)
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        self.day_count(start_time, end_time) / 336.0
    }
}
impl fmt::Display for E283666 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E283666")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use super::E283666;

    fn parse_date(date_str: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date")
    }

    #[test]
    fn test_daycount_twenty_eight_three_thirty_six_1() {
        let local_date1 = parse_date("2019-03-01T00:00:00");
        let local_date2 = parse_date("2019-03-31T00:00:00");
        let result = 27.0;
        assert_eq!(result, E283666::new(None).day_count(local_date1, local_date2) as f64);
    }

    #[test]
    fn test_day_count_fraction_twenty_eight_three_thirty_six_1() {
        let local_date1 = parse_date("2019-03-01T00:00:00");
        let local_date2 = parse_date("2019-03-31T00:00:00");
        let result = 27.0 / 336.0; // 27 divided by 336
        assert_eq!(result, E283666::new(None).day_count_fraction(local_date1, local_date2));
    }

    #[test]
    fn test_daycount_twenty_eight_three_thirty_six_2() {
        let local_date1 = parse_date("2019-03-01T00:00:00");
        let local_date3 = parse_date("2019-04-30T00:00:00");
        let result = 55.0;
        assert_eq!(result, E283666::new(None).day_count(local_date1, local_date3) as f64);
    }

    #[test]
    fn test_day_count_fraction_twenty_eight_three_thirty_six_2() {
        let local_date1 = parse_date("2019-03-01T00:00:00");
        let local_date3 = parse_date("2019-04-30T00:00:00");
        let result = 55.0 / 336.0; // 55 divided by 336
        assert_eq!(result, E283666::new(None).day_count_fraction(local_date1, local_date3));
    }

    #[test]
    fn test_daycount_twenty_eight_three_thirty_six_3() {
        let local_date1 = parse_date("2019-03-01T00:00:00");
        let local_date4 = parse_date("2019-05-30T23:00:00");
        let result = 83.0;
        assert_eq!(result, E283666::new(None).day_count(local_date1, local_date4) as f64);
    }

    #[test]
    fn test_day_count_fraction_twenty_eight_three_thirty_six_3() {
        let local_date1 = parse_date("2019-03-01T00:00:00");
        let local_date4 = parse_date("2019-05-30T23:00:00");
        let result = 83.0 / 336.0; // 83 divided by 336
        assert_eq!(result, E283666::new(None).day_count_fraction(local_date1, local_date4));
    }
}