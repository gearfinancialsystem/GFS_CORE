use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Preceeding::Preceeding;

use crate::subtypes::IsoDatetime::IsoDatetime;
use std::rc::Rc;


#[derive(Debug, Eq, PartialEq)]
pub struct SCP {
    pub scConvention: ShiftCalc,
    pub bdConvention: Preceeding
}

impl SCP {
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        return SCP {    scConvention: ShiftCalc, 
                        bdConvention: Preceeding::new(calendar)};
    }
    pub fn type_str(&self) -> String {
        return "SCP day convention".to_string();
    }
    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }
    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}

impl TraitEnumOptionDescription for SCP {
    fn get_option_rank(&self) -> &str {
        "5"
    }
    fn get_identifier(&self) -> &str {
        "shiftCalculatePreceding"
    }
    fn get_name(&self) -> &str {
        "Shift-Calculate Preceding"
    }
    fn get_acronym(&self) -> &str {
        "SCP"
    }
    fn get_description(&self) -> &str {
        "Shift event dates first then calculate accruals etc. Strictly shift to the most recent preceding business day."
    }
}
