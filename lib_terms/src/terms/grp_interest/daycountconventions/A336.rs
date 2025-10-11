use std::fmt;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::traits::TraitCountConvention::TraitDayCountConvention;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct A336;
impl A336 {
    pub fn new() -> Self {
        A336
    }
}

impl TraitDayCountConvention for A336 {
    /// Calculates the number of days between two dates
    fn day_count(&self, start_time: PhantomIsoDatetimeW, end_time: PhantomIsoDatetimeW) -> f64 {
        end_time.numdays_between_dates(&start_time)
        //(end_time - start_time).num_days() as f64
    }

    /// Calculates the day count fraction between two dates using the A/336 convention
    fn day_count_fraction(&self, start_time: PhantomIsoDatetimeW, end_time: PhantomIsoDatetimeW) -> f64 {
        Self::day_count(&self, start_time, end_time) as f64 / 336.0
    }
}
impl fmt::Display for A336 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A336")
    }
}
// 
// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//     use super::*;
//     use crate::terms::grp_interest::daycountconventions::A336::A336;
//     use lib_types::types::IsoDatetime::IsoDatetime;
//     
//     fn set_vars() -> (IsoDatetime, IsoDatetime, IsoDatetime) {
//         let local_date1 = IsoDatetime::from_str("2019-02-01 00:00:00").expect("");
//         let local_date2 = IsoDatetime::from_str("2019-04-30 00:00:00").expect("");
//         let local_date3 = IsoDatetime::from_str("2019-06-30 00:00:00").expect("");
//         (local_date1, local_date2, local_date3)
//     }
//     
// 
//     //let local_date2 = parse_date("2019-04-30T00:00:00");
// 
//     #[test]
//     fn test_day_count_actual_three_thirty_six_1() {
//         let (local_date1, local_date2, _) = set_vars();
//         let result = 88.0;
//         assert_eq!(result, A336.day_count(local_date1, local_date2));
//     }
// 
//     #[test]
//     fn test_day_count_fraction_actual_three_thirty_six_1() {
//         let (local_date1, local_date2, _) = set_vars();
//         let result = 0.2619047619047619; // 88 divided by 336
//         assert_eq!(result, A336.day_count_fraction(local_date1, local_date2));
//     }
// 
//     #[test]
//     fn test_day_count_actual_three_thirty_six_2() {
//         let (local_date1, _, local_date3) = set_vars();
//         let result = 149.0;
//         assert_eq!(result, A336.day_count(local_date1, local_date3));
//     }
// 
//     #[test]
//     fn test_day_count_fraction_actual_three_thirty_six_2() {
//         let (local_date1, _, local_date3) = set_vars();
//         let result = 0.44345238095238093; // 149 divided by 336
//         assert_eq!(result, A336.day_count_fraction(local_date1, local_date3));
//     }
// }