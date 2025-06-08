use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::ModifiedPreceeding::ModifiedPreceeding;

use crate::subtypes::IsoDatetime::IsoDatetime;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub struct CSMP {
    pub scConvention: CalcShift,
    pub bdConvention: ModifiedPreceeding
}

impl CSMP {
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        return CSMP {    scConvention: CalcShift, 
                        bdConvention: ModifiedPreceeding::new(calendar)};
    }
    pub fn type_str(&self) -> String {
        return "CSMP day convention".to_string();
    }
    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}

