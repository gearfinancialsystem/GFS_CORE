use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use chrono::NaiveDateTime;

pub trait TraitShiftCalcConvention {
    fn shift(&self, time: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime;
}