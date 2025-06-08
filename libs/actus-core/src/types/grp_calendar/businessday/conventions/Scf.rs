use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Following::Following;

use std::rc::Rc;


#[derive(Debug, Eq, PartialEq)]
pub struct SCF {
    pub scConvention: ShiftCalc,
    pub bdConvention: Following
}

impl SCF {
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        SCF {
            scConvention: ShiftCalc,
            bdConvention: Following::new(calendar),
        }
    }
    pub fn type_str(&self) -> String {
        return "SCF day convention".to_string();
    }
    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}

impl TraitEnumOptionDescription for SCF {
    fn get_option_rank(&self) -> &str {
        "1"
    }
    fn get_identifier(&self) -> &str {
        "shiftCalculateFollowing"
    }
    fn get_name(&self) -> &str {
        "Shift-Calculate Following"
    }
    fn get_acronym(&self) -> &str {
        "SCF"
    }
    fn get_description(&self) -> &str {
        "Shift event dates first then calculate accruals etc. Strictly shift to the next following business day."
    }
}    