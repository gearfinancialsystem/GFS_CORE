use std::fmt;
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use crate::traits::TraitEndOfMonthConvention::TraitEndOfMonthConvention;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct SD;

impl SD {
    pub fn new() -> Self {
        SD
    }
}

impl TraitEndOfMonthConvention for SD {
    /// Returns the input date without any changes
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime {
        *date
    }
}

impl fmt::Display for SD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "SD")
    }
}
