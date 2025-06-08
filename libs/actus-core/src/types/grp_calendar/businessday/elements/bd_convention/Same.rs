use crate::subtypes::IsoDatetime::IsoDatetime;


#[derive(Debug, Eq, PartialEq)]
pub struct Same;

impl Same {
    pub fn new() -> Self {
        Same
    }
}

impl BusinessDayConventionTrait for Same {
    /// Returns the non-shifted date (even if a non-business day)
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime {
        *date
    }
}
