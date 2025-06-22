use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use chrono::NaiveDateTime;

pub trait TraitShiftCalcConvention {
    fn shift(&self, time: &NaiveDateTime, convention: &dyn TraitBusinessDayAdjuster) -> NaiveDateTime;
}