
use crate::types::IsoDatetime::IsoDatetime;

pub trait TraitCycleAdjuster {
    fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime;
    fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime;
}
