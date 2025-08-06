use std::fmt;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

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
    fn shift(&self, time: &PhantomIsoDatetimeW, convention: &dyn TraitBusinessDayAdjuster) -> PhantomIsoDatetimeW {
        convention.shift(time)
    }
}

impl fmt::Display for ShiftCalc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShiftCalc")
    }
}