use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedFollowing::ModifiedFollowing;

use crate::subtypes::IsoDatetime::IsoDatetime;
use std::rc::Rc;


#[derive(Debug, Eq, PartialEq)]
pub struct SCMF {
    pub scConvention: ShiftCalc,
    pub bdConvention: ModifiedFollowing
}

impl SCMF {
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        return SCMF {   scConvention: ShiftCalc, 
                        bdConvention: ModifiedFollowing::new(calendar)};
    }
    pub fn type_str(&self) -> String {
        return "SCMF day convention".to_string();
    }
    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}

impl TraitEnumOptionDescription for SCMF {
    fn get_option_rank(&self) -> &str {
        "2"
    }
    fn get_identifier(&self) -> &str {
        "shiftCalculateModifiedFollowing"
    }
    fn get_name(&self) -> &str {
        "Shift-Calculate Modified-Following"
    }
    fn get_acronym(&self) -> &str {
        "SCMF"
    }
    fn get_description(&self) -> &str {
        "Shift event dates first then calculate accruals etc. Shift to the next following business day if this falls in the same month. Shift to the most recent preceding business day otherwise."
    }
}   