use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

pub trait TraitShiftCalcConvention {
    fn shift(&self, time: &PhantomIsoDatetimeW, convention: &dyn TraitBusinessDayAdjuster) -> PhantomIsoDatetimeW;
}