use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedPreceeding::ModifiedPreceeding;
use crate::terms::grp_calendar::Calendar::Calendar;

use chrono::NaiveDateTime;
use std::rc::Rc;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CSMP {
    pub scConvention: CalcShift,
    pub bdConvention: ModifiedPreceeding
}

impl CSMP {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        return CSMP {    scConvention: CalcShift, 
                        bdConvention: ModifiedPreceeding::new(calendar)};
    }
    pub fn type_str(&self) -> String {
        return "CSMP day convention".to_string();
    }
    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}

