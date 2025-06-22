use std::rc::Rc;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Same::Same;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use crate::types::isoDatetime::IsoDatetime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NOS {
    pub scConvention: ShiftCalc,
    pub bdConvention: Same
}

impl NOS {
    pub fn new(_calendar: Rc<Calendar>) -> Self {
        NOS { scConvention: ShiftCalc, bdConvention: Same}
    }
    pub fn type_str(&self) -> String {
        return "NOS day convention".to_string();
    }

    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn TraitBusinessDayAdjuster) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}

