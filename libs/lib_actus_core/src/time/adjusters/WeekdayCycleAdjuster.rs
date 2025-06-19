use crate::types::isoDatetime::{traitNaiveDateTimeExtension, IsoDatetime};
use crate::util::CycleUtils::CycleUtils;
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use crate::traits::TraitCycleAdjuster::TraitCycleAdjuster;
use chrono::{Months, NaiveDate, Weekday};
use chrono::Datelike;


pub struct WeekdayCycleAdjuster {
    pub day_of_week: Weekday,
    pub position: i32,
    pub stub: char,
}

impl WeekdayCycleAdjuster {
    pub fn new(cycle: &String) -> Result<Self, AttributeConversionException> {
        let weekday = CycleUtils::parse_weekday(cycle);
        let position = CycleUtils::parse_position(cycle);
        let stub = CycleUtils::parse_stub(cycle);

        match (weekday, position, stub) {
            (Ok(val_day_of_week), Ok(val_position), Ok(val_stub)) => {
                Ok( WeekdayCycleAdjuster {day_of_week: val_day_of_week, position: val_position as i32, stub: val_stub })
            }
            _ => {
                Err(AttributeConversionException)
            }
        }
    }
}

impl TraitCycleAdjuster for WeekdayCycleAdjuster {
    fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        let test1 = time.checked_add_months(Months::new(1)).unwrap();
        let test2 = NaiveDate::from_weekday_of_month_opt(test1.year(), test1.month(), self.day_of_week, self.position as u8);
        test2.unwrap().and_hms_opt(0,0,0).unwrap()
    }

    fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        let test1 = time.checked_sub_months(Months::new(1)).unwrap();
        let test2 = NaiveDate::from_weekday_of_month_opt(test1.year(), test1.month(), self.day_of_week, self.position as u8);
        test2.unwrap().and_hms_opt(0,0,0).unwrap()
    }
}

#[cfg(test)]
mod tests_period_cycle_adjuster {
    use crate::time::adjusters::PeriodCycleAdjuster::PeriodCycleAdjuster;
    use crate::types::isoDatetime::IsoDatetime;
    use super::*;
    #[test]
    fn test_plus_1MonShort(){
        let mut w_adjuster = WeekdayCycleAdjuster::new(&"1MonL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-02-04T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.plus_cycle(t0) ;
        println!("{:?}", test);
        assert_eq!(t1, test);

    }

    #[test]
    fn test_minus_1MonShort(){
        let mut w_adjuster = WeekdayCycleAdjuster::new(&"1MonL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2018-12-03T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_plus_1FriShort(){
        let mut w_adjuster = WeekdayCycleAdjuster::new(&"1FriL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-07-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-08-02T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_minus_1FriShort() {
        let mut w_adjuster = WeekdayCycleAdjuster::new(&"1FriL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-07-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-06-07T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_plus_3SatShort() {
        let mut w_adjuster = WeekdayCycleAdjuster::new(&"3SatL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-11-16T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_minus_3SatShort() {
        let mut w_adjuster = WeekdayCycleAdjuster::new(&"3SatL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-09-21T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
}

