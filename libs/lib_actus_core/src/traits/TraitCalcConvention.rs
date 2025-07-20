use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::types::IsoDatetime::IsoDatetime;


pub trait TraitShiftCalcConvention {
    fn shift(&self, time: &IsoDatetime, convention: &dyn TraitBusinessDayAdjuster) -> IsoDatetime;
}