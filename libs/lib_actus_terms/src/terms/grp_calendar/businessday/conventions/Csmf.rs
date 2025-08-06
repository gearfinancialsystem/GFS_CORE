use std::fmt;
use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedFollowing::ModifiedFollowing;
use crate::terms::grp_calendar::Calendar::Calendar;

use std::rc::Rc;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

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

    pub fn shift_sc(&self, date: &PhantomIsoDatetimeW, convention: &dyn TraitBusinessDayAdjuster) -> PhantomIsoDatetimeW {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &PhantomIsoDatetimeW) -> PhantomIsoDatetimeW {
        self.bdConvention.shift(date)
    }
}


impl fmt::Display for CSMF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSMF (scConvention: {}, bdConvention: {})", self.scConvention.to_string(), self.bdConvention.to_string() )
    }
}