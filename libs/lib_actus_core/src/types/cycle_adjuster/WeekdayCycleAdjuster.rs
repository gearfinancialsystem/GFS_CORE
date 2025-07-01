use crate::types::IsoDatetime::IsoDatetime;
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use crate::traits::TraitCycleAdjuster::TraitCycleAdjuster;
use chrono::{Months, NaiveDate, Weekday};
use chrono::Datelike;
use crate::types::IsoCycle::{LONG_STUB, SHORT_STUB};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeekdayCycleAdjuster {
    pub day_of_week: Weekday,
    pub position: i32,
    pub stub: char,
}

impl WeekdayCycleAdjuster {
    pub fn new(cycle: String) -> Result<Self, AttributeConversionException> {
        let weekday = Self::parse_weekday(cycle.clone());
        let position = Self::parse_position(cycle.clone());
        let stub = Self::parse_stub(cycle);

        match (weekday, position, stub) {
            (Ok(val_day_of_week), Ok(val_position), Ok(val_stub)) => {
                Ok( WeekdayCycleAdjuster {day_of_week: val_day_of_week, position: val_position as i32, stub: val_stub })
            }
            _ => {
                Err(AttributeConversionException)
            }
        }
    }

    pub fn parse_weekday(cycle: String) -> Result<Weekday, AttributeConversionException> {
        let weekday_part = cycle.split('L').next().unwrap();
        let weekday_str = &weekday_part[1..4]; // Assuming the format is like "1MONL"
        match weekday_str {
            "Mon" => Ok(Weekday::Mon),
            "Tue" => Ok(Weekday::Tue),
            "Wed" => Ok(Weekday::Wed),
            "Thu" => Ok(Weekday::Thu),
            "Fri" => Ok(Weekday::Fri),
            "Sat" => Ok(Weekday::Sat),
            "Sun" => Ok(Weekday::Sun),
            _ => Err(AttributeConversionException),
        }
    }


    pub fn parse_position(cycle: String) -> Result<u32, AttributeConversionException> {
        let position_char = cycle.chars().next().ok_or(AttributeConversionException)?;
        position_char.to_digit(10).ok_or(AttributeConversionException)
    }


    pub fn parse_stub(cycle: String) -> Result<char, AttributeConversionException> {
        let stub_part = cycle.split('L').nth(1).ok_or(AttributeConversionException)?;
        let stub = stub_part.chars().next().ok_or(AttributeConversionException)?;
        if stub == LONG_STUB || stub == SHORT_STUB {
            Ok(stub)
        } else {
            Err(AttributeConversionException)
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
    use crate::types::cycle_adjuster::PeriodCycleAdjuster::PeriodCycleAdjuster;
    use crate::types::IsoDatetime::IsoDatetime;
    use super::*;
    #[test]
    fn test_plus_1MonShort(){
        let mut w_adjuster = WeekdayCycleAdjuster::new("1MonL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-02-04T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.plus_cycle(t0) ;
        println!("{:?}", test);
        assert_eq!(t1, test);

    }

    #[test]
    fn test_minus_1MonShort(){
        let mut w_adjuster = WeekdayCycleAdjuster::new("1MonL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2018-12-03T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_plus_1FriShort(){
        let mut w_adjuster = WeekdayCycleAdjuster::new("1FriL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-07-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-08-02T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_minus_1FriShort() {
        let mut w_adjuster = WeekdayCycleAdjuster::new("1FriL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-07-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-06-07T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_plus_3SatShort() {
        let mut w_adjuster = WeekdayCycleAdjuster::new("3SatL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-11-16T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.plus_cycle(t0) ;
        assert_eq!(t1, test);
    }

    #[test]
    fn test_minus_3SatShort() {
        let mut w_adjuster = WeekdayCycleAdjuster::new("3SatL1".to_string()).unwrap();
        let mut t0 = IsoDatetime::parse_from_str("2019-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let mut t1 = IsoDatetime::parse_from_str("2019-09-21T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut test = w_adjuster.minus_cycle(t0) ;
        assert_eq!(t1, test);
    }
}

