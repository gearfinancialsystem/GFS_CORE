use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Preceeding::Preceeding;

use chrono::NaiveDateTime;
use std::rc::Rc;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct CSP {
    pub scConvention: CalcShift,
    pub bdConvention: Preceeding
}

impl CSP {
    pub fn new(calendar: Rc<dyn TraitBusinessDayCalendar>) -> Self {
        return CSP {    scConvention: CalcShift, 
                        bdConvention: Preceeding::new(calendar)};
    }
    pub fn type_str(&self) -> String {
        return "CSP day convention".to_string();
    }
    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}

impl TraitEnumOptionDescription for CSP {
    fn get_option_rank(&self) -> &str {
        "7"
    }
    fn get_identifier(&self) -> &str {
        "calculateShiftPreceding"
    }
    fn get_name(&self) -> &str {
        "Calculate-Shift Preceding"
    }
    fn get_acronym(&self) -> &str {
        "CSP"
    }
    fn get_description(&self) -> &str {
        "Calculate accruals etc. first then shift event dates. Strictly shift to the most recent preceding business day."
    }
}
