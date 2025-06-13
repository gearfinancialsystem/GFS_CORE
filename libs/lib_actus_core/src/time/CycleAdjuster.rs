use crate::types::isoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;
use crate::traits::TraitCycleAdjuster::TraitCycleAdjuster;
use crate::time::adjusters::{PeriodCycleAdjuster::PeriodCycleAdjuster,
                             WeekdayCycleAdjuster::WeekdayCycleAdjuster};


struct CycleAdjuster {
    adjuster: Box<dyn TraitCycleAdjuster>,
}

impl CycleAdjuster {
    pub fn new(cycle: &String) -> Self {
        if CycleUtils::is_period(cycle) {
            CycleAdjuster {
                adjuster: Box::new(PeriodCycleAdjuster::new(cycle).unwrap()),
            }
        } else {
            CycleAdjuster {
                adjuster: Box::new(WeekdayCycleAdjuster::new(cycle).unwrap()),
            }
        }
    }

    pub fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        self.adjuster.plus_cycle(time)
    }

    pub fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        self.adjuster.minus_cycle(time)
    }
}
