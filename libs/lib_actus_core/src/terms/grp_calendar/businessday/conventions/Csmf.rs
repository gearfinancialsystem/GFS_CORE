use std::fmt;
use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedFollowing::ModifiedFollowing;
use crate::terms::grp_calendar::Calendar::Calendar;

use std::rc::Rc;
use crate::terms::grp_calendar::businessday::conventions::Csf::CSF;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CSMF {
    pub scConvention: CalcShift,
    pub bdConvention: ModifiedFollowing
}

impl CSMF {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        return CSMF {    scConvention: CalcShift, 
                        bdConvention: ModifiedFollowing::new(calendar)};
    }

    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn TraitBusinessDayAdjuster) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}


impl fmt::Display for CSMF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSMF (scConvention: {}, bdConvention: {})", self.scConvention.to_string(), self.bdConvention.to_string() )
    }
}