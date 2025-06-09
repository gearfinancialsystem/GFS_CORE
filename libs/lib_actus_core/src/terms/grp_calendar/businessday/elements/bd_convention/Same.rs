use chrono::NaiveDateTime;
use crate::traits::TraitBusinessDayConvention::TraitBusinessDayConvention;

#[derive(Debug, Eq, PartialEq)]
pub struct Same;

impl Same {
    pub fn new() -> Self {
        Same
    }
}

impl TraitBusinessDayConvention for Same {
    /// Returns the non-shifted date (even if a non-business day)
    fn shift(&self, date: &NaiveDateTime) -> NaiveDateTime {
        *date
    }
}
