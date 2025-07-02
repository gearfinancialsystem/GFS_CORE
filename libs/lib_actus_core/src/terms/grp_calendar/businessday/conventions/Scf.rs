use std::fmt;
use std::rc::Rc;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Following::Following;

use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCF {
    pub scConvention: ShiftCalc,
    pub bdConvention: Following
}

impl SCF {
    pub fn new(calendar: Rc<Calendar>) -> Self {
        SCF {
            scConvention: ShiftCalc,
            bdConvention: Following::new(calendar),
        }
    }
    pub fn type_str(&self) -> String {
        return "SCF day convention".to_string();
    }
    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn TraitBusinessDayAdjuster) -> IsoDatetime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}
impl fmt::Display for SCF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SCF (scConvention: {}, bdConvention: {})", self.scConvention.to_string(), self.bdConvention.to_string() )
    }
}
