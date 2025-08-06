use std::fmt;
use std::rc::Rc;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Same::Same;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NOS {
    pub scConvention: ShiftCalc,
    pub bdConvention: Same
}

impl NOS {
    pub fn new(_calendar: Rc<Calendar>) -> Self {
        NOS { scConvention: ShiftCalc, bdConvention: Same}
    }

    pub fn shift_sc(&self, date: &PhantomIsoDatetimeW, convention: &dyn TraitBusinessDayAdjuster) -> PhantomIsoDatetimeW {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &PhantomIsoDatetimeW) -> PhantomIsoDatetimeW {
        self.bdConvention.shift(date)
    }
}


impl fmt::Display for NOS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NOS (scConvention: {}, bdConvention: {})", self.scConvention.to_string(), self.bdConvention.to_string() )
    }
}