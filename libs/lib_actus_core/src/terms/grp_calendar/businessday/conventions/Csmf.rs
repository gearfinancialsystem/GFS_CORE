use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedFollowing::ModifiedFollowing;
use crate::terms::grp_calendar::Calendar::Calendar;

use chrono::NaiveDateTime;
use std::rc::Rc;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;

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
    pub fn type_str(&self) -> String {
        return "CSMF day convention".to_string();
    }
    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}


