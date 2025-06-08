use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedPreceeding::ModifiedPreceeding;

use crate::subtypes::IsoDatetime::IsoDatetime;
use std::rc::Rc;


#[derive(Debug, Eq, PartialEq)]
pub struct SCMP {
    pub scConvention: ShiftCalc,
    pub bdConvention: ModifiedPreceeding
}

impl SCMP {
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        return SCMP {    scConvention: ShiftCalc, 
                        bdConvention: ModifiedPreceeding::new(calendar)};
    }
    pub fn type_str(&self) -> String {
        return "SCMP day convention".to_string();
    }
    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }
    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}


impl TraitEnumOptionDescription for SCMP {
    fn get_option_rank(&self) -> &str {
        "6"
    }
    fn get_identifier(&self) -> &str {
        "shiftCalculateModifiedPreceding"
    }
    fn get_name(&self) -> &str {
        "Shift-Calculate Modified-Preceding"
    }
    fn get_acronym(&self) -> &str {
        "SCMP"
    }
    fn get_description(&self) -> &str {
        "Shift event dates first then calculate accruals etc. Shift to the most recent preceding business day if this falls in the same month. Shift to the next following business day otherwise."
    }
}    