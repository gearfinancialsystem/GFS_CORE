use crate::subtypes::IsoDatetime::IsoDatetime;




#[derive(Debug, Eq, PartialEq)]
pub struct CalcShift;

impl CalcShift {
    /// Constructor
    pub fn new(&self) -> Self {
        CalcShift
    }
}

impl ShiftCalcConventionTrait for CalcShift {
    /// Returns the `time` unshifted
    fn shift(&self, time: &IsoDatetime, _convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
        *time
    }
    
}
