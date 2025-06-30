use crate::types::IsoDatetime::IsoDatetime;
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime {
        *date
    }
}

