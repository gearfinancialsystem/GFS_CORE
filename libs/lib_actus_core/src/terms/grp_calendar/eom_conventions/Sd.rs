
use chrono::NaiveDateTime;
use chrono::{Datelike, Duration, Timelike};
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SD;

impl SD {
    pub fn new() -> Self {
        SD
    }
    pub fn type_str(&self) -> String {
        "SD eom".to_string()
    }
}

impl TraitEndOfMonthConvention for SD {
    /// Returns the input date without any changes
    fn shift(&self, date: &NaiveDateTime) -> NaiveDateTime {
        *date
    }
}

