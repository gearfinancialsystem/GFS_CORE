use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Same;

impl Same {
    pub fn new() -> Self {
        Same
    }
}

impl TraitBusinessDayAdjuster for Same {
    /// Returns the non-shifted date (even if a non-business day)
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime {
        *date
    }
}
