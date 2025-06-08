use crate::subtypes::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::businessday::elements::sc_convention::CalcShift::CalcShift;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Following::Following;

use std::rc::Rc;


#[derive(Debug, Eq, PartialEq)]
pub struct CSF {
    pub scConvention: CalcShift,
    pub bdConvention: Following,
}

impl CSF {
    /// Construit un `CSF` en prenant possession du calendrier (boxed).
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
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
    pub fn shift_sc(&self, date: &IsoDatetime, convention: &dyn BusinessDayConventionTrait) -> crate::subtypes::IsoDatetime::IsoDatetime {
        self.scConvention.shift(date, convention)
    }

    /// Applique le décalage via la BDC locale (bdConvention)
    pub fn shift_bd(&self, date: &IsoDatetime) -> IsoDatetime {
        self.bdConvention.shift(date)
    }
}

impl TraitEnumOptionDescription for CSF {
    fn get_option_rank(&self) -> &str {
        "3"
    }
    fn get_identifier(&self) -> &str {
        "calculateShiftFollowing"
    }
    fn get_name(&self) -> &str {
        "Calculate-Shift Following"
    }
    fn get_acronym(&self) -> &str {
        "CSF"
    }
    fn get_description(&self) -> &str {
        "Calculate accruals etc. first then shift event dates. Strictly shift to the next following business day."
    }
}    