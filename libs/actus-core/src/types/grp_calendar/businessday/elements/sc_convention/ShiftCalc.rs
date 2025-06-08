
use crate::subtypes::IsoDatetime::IsoDatetime;

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

impl ShiftCalcConventionTrait for ShiftCalc {
    /// Returns the `time` shifted according to the respective `BusinessDayConvention`
    fn shift(&self, time: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
        convention.shift(time)
    }
}
