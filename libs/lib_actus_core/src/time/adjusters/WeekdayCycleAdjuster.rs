use crate::types::isoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use crate::util::CycleUtils::CycleUtils;
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use crate::traits::TraitCycleAdjuster::TraitCycleAdjuster;
use chrono::Weekday;

pub struct WeekdayCycleAdjuster {
    pub day_of_week: Weekday,
    pub period: IsoPeriod,
    pub stub: char,
}

impl WeekdayCycleAdjuster {
    pub fn new(cycle: &String) -> Result<Self, AttributeConversionException> {
        let weekday = CycleUtils::parse_weekday(cycle);
        let period = CycleUtils::parse_period(cycle);
        let stub = CycleUtils::parse_stub(cycle);

        match (weekday, period, stub) {
            (Ok(val_day_of_week), Ok(val_period), Ok(val_stub)) => {
                Ok( WeekdayCycleAdjuster {day_of_week: val_day_of_week, period: val_period, stub: val_stub })
            }
            _ => {
                Err(AttributeConversionException)
            }
        }
    }
}

impl TraitCycleAdjuster for WeekdayCycleAdjuster {
    fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        time + self.period.clone()
    }

    fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        time - self.period.clone()
    }
}