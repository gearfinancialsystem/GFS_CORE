use std::rc::Rc;
use chrono::NaiveDateTime;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Same::Same;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NOS {
    pub scConvention: ShiftCalc,
    pub bdConvention: Same
}

impl NOS {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        return NOS { scConvention: ShiftCalc, bdConvention: Same};
    }
    pub fn type_str(&self) -> String {
        return "NOS day convention".to_string();
    }

    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}

