use std::fmt;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct CalcShift;

impl CalcShift {
    /// Constructor
    pub fn new(&self) -> Self {
        CalcShift
    }
}

impl TraitShiftCalcConvention for CalcShift {
    /// Returns the `time` unshifted
    fn shift(&self, time: &IsoDatetime, _convention: &dyn TraitBusinessDayAdjuster) -> IsoDatetime {
        *time
    }
    
}

impl fmt::Display for CalcShift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CalcShift")
    }
}