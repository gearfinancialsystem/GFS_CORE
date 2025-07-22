use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use lib_actus_types::types::IsoDatetime::IsoDatetime;


pub trait TraitShiftCalcConvention {
    fn shift(&self, time: &IsoDatetime, convention: &dyn TraitBusinessDayAdjuster) -> IsoDatetime;
}