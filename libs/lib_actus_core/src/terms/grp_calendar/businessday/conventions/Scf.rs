use chrono::NaiveDateTime;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Following::Following;

use std::rc::Rc;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct SCF {
    pub scConvention: ShiftCalc,
    pub bdConvention: Following
}

impl SCF {
    pub fn new(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        SCF {
            scConvention: ShiftCalc,
            bdConvention: Following::new(calendar),
        }
    }
    pub fn type_str(&self) -> String {
        return "SCF day convention".to_string();
    }
    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}

