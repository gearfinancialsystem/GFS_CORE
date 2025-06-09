use chrono::NaiveDateTime;
use crate::terms::grp_calendar::businessday::elements::sc_convention::ShiftCalc::ShiftCalc;
use crate::terms::grp_calendar::businessday::elements::bd_convention::Same::Same;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct NOS {
    pub scConvention: ShiftCalc,
    pub bdConvention: Same
}

impl NOS {
    pub fn new() -> Self {
        return NOS { scConvention: ShiftCalc, bdConvention: Same};
    }
    pub fn type_str(&self) -> String {
        return "NOS day convention".to_string();
    }

    pub fn shift_sc(&self, date: &NaiveDateTime, convention: &dyn TraitBusinessDayConvention) -> NaiveDateTime {
        self.scConvention.shift(date, convention)
    }

    pub fn shift_bd(&self, date: &NaiveDateTime) -> NaiveDateTime {
        self.bdConvention.shift(date)
    }
}

impl TraitEnumOptionDescription for NOS {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "noShift"
    }
    fn get_name(&self) -> &str {
        "No Shift"
    }
    fn get_acronym(&self) -> &str {
        "NOS"
    }
    fn get_description(&self) -> &str {
        "No shift applied to non-business days."
    }
}    