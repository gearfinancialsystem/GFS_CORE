use crate::types::isoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use crate::util::CycleUtils::CycleUtils;
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use crate::traits::TraitCycleAdjuster::TraitCycleAjuster;

pub struct PeriodCycleAdjuster {
    pub period: IsoPeriod,
    pub stub: char,
}

impl PeriodCycleAdjuster {
    pub fn new(cycle: &String) -> Result<Self, AttributeConversionException> {
        let period = CycleUtils::parse_period(cycle);
        let stub = CycleUtils::parse_stub(cycle);

        match (period, stub) {
            (Ok(val_period), Ok(val_stub)) => {
                println!("Both results are Ok: {} and {}", val_period, val_stub);
                Ok(PeriodCycleAdjuster { period: val_period, stub: val_stub })
            }
            (Err(e), _) | (_, Err(e)) => {
                Err(e)
            }
        }
    }
}

impl TraitCycleAjuster for PeriodCycleAdjuster {
    fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        time + self.period.clone()
    }

    fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        time - self.period.clone()
    }
}