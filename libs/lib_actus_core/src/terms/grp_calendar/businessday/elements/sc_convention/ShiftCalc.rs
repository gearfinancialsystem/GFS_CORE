use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use crate::types::IsoDatetime::IsoDatetime;

/// Component that represents the Shift-first-Calculate-Second convention
///
/// This convention assumes that when shifting the event time according
/// to a `BusinessDayAdjuster`, the time is shifted first and calculations
/// are performed thereafter. Hence, calculations are based on the shifted time as well.

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct ShiftCalc;

impl ShiftCalc {
    pub fn new() -> Self {
        ShiftCalc
    }
}

impl TraitShiftCalcConvention for ShiftCalc {
    /// Returns the `time` shifted according to the respective `BusinessDayAdjuster`
    fn shift(&self, time: &IsoDatetime, convention: &dyn TraitBusinessDayAdjuster) -> IsoDatetime {
        convention.shift(time)
    }
}
