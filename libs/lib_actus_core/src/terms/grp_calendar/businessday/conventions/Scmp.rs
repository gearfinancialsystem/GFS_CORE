use std::rc::Rc;

use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedPreceeding::ModifiedPreceeding;
use crate::terms::grp_calendar::Calendar::Calendar;

use chrono::NaiveDateTime;

use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCMP{
    pub scConvention: ShiftCalc,
    pub bdConvention: ModifiedPreceeding
}

impl SCMP {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        return SCMP {    scConvention: ShiftCalc, 
                        bdConvention: ModifiedPreceeding::new(calendar)};
    }
    pub fn type_str(&self) -> String {
        return "SCMP day convention".to_string();
    }
    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }
    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}


