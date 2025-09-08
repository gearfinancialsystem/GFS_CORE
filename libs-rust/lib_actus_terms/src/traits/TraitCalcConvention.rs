use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

pub trait TraitShiftCalcConvention {
    fn shift(&self, time: &PhantomIsoDatetimeW, convention: &dyn TraitBusinessDayAdjuster) -> PhantomIsoDatetimeW;
}