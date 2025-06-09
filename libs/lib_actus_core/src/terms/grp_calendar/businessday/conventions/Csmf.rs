use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedFollowing::ModifiedFollowing;

use chrono::NaiveDateTime;
use std::rc::Rc;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct CSMF {
    pub scConvention: CalcShift,
    pub bdConvention: ModifiedFollowing
}

impl CSMF {
    pub fn new(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
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


impl TraitEnumOptionDescription for CSMF {
    fn get_option_rank(&self) -> &str {
        "4"
    }
    fn get_identifier(&self) -> &str {
        "calculateShiftModifiedFollowing"
    }
    fn get_name(&self) -> &str {
        "Calculate-Shift Modified-Following"
    }
    fn get_acronym(&self) -> &str {
        "CSMF"
    }
    fn get_description(&self) -> &str {
        "Calculate accruals etc. first then shift event dates. Shift to the next following business day if this falls in the same month. Shift to the most recent preceding business day otherwise."
    }
}    