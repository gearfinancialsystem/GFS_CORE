use crate::cycle::cycle::AttributeConversionException;
use crate::local_datetime::local_datetime::IsoDateTime;
use crate::period::period::IsoPeriod;
use super::super::cycle::traitCycleAjuster::TraitCycleAjuster;
use super::super::cycle::cycle::CycleUtils;

struct PeriodCycleAdjuster {
    period: IsoPeriod,
    stub: char,
}

impl PeriodCycleAdjuster {
    fn new(cycle: &String) -> Result<Self, AttributeConversionException> {
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
    fn plus_cycle(&self, time: IsoDateTime) -> IsoDateTime {
        time + self.period
    }

    fn minus_cycle(&self, time: IsoDateTime) -> IsoDateTime {
        time - self.period
    }
}