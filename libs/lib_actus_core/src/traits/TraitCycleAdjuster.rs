
use crate::types::isoDatetime::IsoDatetime;

pub trait TraitCycleAjuster {
    fn plus_cycle(&self, time: IsoDatetime) -> IsoDatetime;
    fn minus_cycle(&self, time: IsoDatetime) -> IsoDatetime;
}
