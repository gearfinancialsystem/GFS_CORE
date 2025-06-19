use std::rc::Rc;

use chrono::NaiveDateTime;
use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Following::Following;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitCalcConvention::TraitShiftCalcConvention;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CSF {
    pub scConvention: CalcShift,
    pub bdConvention: Following,
}

impl CSF {
    /// Construit un `CSF` en prenant possession du calendrier (boxed).
    pub fn new(calendar: Rc<Calendar>) -> Self {
        CSF {
            scConvention: CalcShift,
            bdConvention: Following::new(calendar),
        }
    }

    pub fn type_str(&self) -> String {
        "CSF day convention".to_string()
    }
    
    /// Applique le décalage selon la convention de shift (scConvention)
    /// en passant un trait object pour la BDC.
    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    /// Applique le décalage via la BDC locale (bdConvention)
    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}

