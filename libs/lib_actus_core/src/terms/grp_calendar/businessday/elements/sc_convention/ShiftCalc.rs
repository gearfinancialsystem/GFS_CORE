
use chrono::NaiveDateTime;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;

/// Component that represents the Shift-first-Calculate-Second convention
///
/// This convention assumes that when shifting the event time according
/// to a `BusinessDayConvention`, the time is shifted first and calculations
/// are performed thereafter. Hence, calculations are based on the shifted time as well.

#[derive(Debug, Eq, PartialEq)]
pub struct ShiftCalc;

impl ShiftCalc {
    pub fn new() -> Self {
        ShiftCalc
    }
}

impl TraitShiftCalcConvention for ShiftCalc {
    /// Returns the `time` shifted according to the respective `BusinessDayConvention`
    fn shift(&self, time: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        convention.shift(time)
    }
}
