use crate::types::isoDatetime::IsoDatetime;
use crate::util::CycleUtils::CycleUtils;
use crate::traits::TraitCycleAdjuster::TraitCycleAjuster;
use crate::time::adjusters::{PeriodCycleAdjuster, WeekdayCycleAdjuster};


struct CycleAdjuster {
    adjuster: Box<dyn TraitCycleAjuster>,
}

impl CycleAdjuster {
    fn new(cycle: &String) -> Self {
        if CycleUtils::is_period(cycle) {
            CycleAdjuster {
                adjuster: Box::new(PeriodCycleAdjuster::new(cycle)),
            }
        } else {
            CycleAdjuster {
                adjuster: Box::new(WeekdayCycleAdjuster::new(cycle)),
            }
        }
    }

    fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        self.adjuster.plus_cycle(time)
    }

    fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime {
        self.adjuster.minus_cycle(time)
    }
}
