use chrono::NaiveDateTime;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;

#[derive(Debug, Eq, PartialEq)]
pub struct CalcShift;

impl CalcShift {
    /// Constructor
    pub fn new(&self) -> Self {
        CalcShift
    }
}

impl TraitShiftCalcConvention for CalcShift {
    /// Returns the `time` unshifted
    fn shift(&self, time: &NaiveDateTime, _convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        *time
    }
    
}
