use std::fmt;
use std::rc::Rc;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedFollowing::ModifiedFollowing;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCMF {
    pub scConvention: ShiftCalc,
    pub bdConvention: ModifiedFollowing
}

impl SCMF {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        return SCMF {   scConvention: ShiftCalc, 
                        bdConvention: ModifiedFollowing::new(calendar)};
    }

    pub fn shift_sc(&self, date: &PhantomIsoDatetimeW, convention: &dyn TraitBusinessDayAdjuster) -> PhantomIsoDatetimeW {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &PhantomIsoDatetimeW) -> PhantomIsoDatetimeW {
        self.bdConvention.shift(date)
    }
}

impl fmt::Display for SCMF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SCMF (scConvention: {}, bdConvention: {})", self.scConvention.to_string(), self.bdConvention.to_string() )
    }
}