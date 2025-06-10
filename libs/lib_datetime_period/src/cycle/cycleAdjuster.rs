// Struct to hold the adjuster
use super::super::cycle::traitCycleAjuster::TraitCycleAjuster;
use super::super::cycle::cycle::CycleUtils;
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

    fn plus_cycle(&self, time: DateTime<Local>) -> DateTime<Local> {
        self.adjuster.plus_cycle(time)
    }

    fn minus_cycle(&self, time: DateTime<Local>) -> DateTime<Local> {
        self.adjuster.minus_cycle(time)
    }
}
