
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

impl TraitEnumOptionDescription for SD {
    fn get_option_rank(&self) -> &str {
        "0"
    }
    fn get_identifier(&self) -> &str {
        "sameDay"
    }
    fn get_name(&self) -> &str {
        "Same Day"
    }
    fn get_acronym(&self) -> &str {
        "SD"
    }
    fn get_description(&self) -> &str {
        "Schedule times always fall on the schedule anchor date day of the month."
    }
}