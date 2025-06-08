
use crate::subtypes::IsoDatetime::IsoDatetime;
use chrono::Duration;
use std::ptr;
use std::rc::Rc;

#[derive(Debug)]
pub struct Following {
    pub calendar: Rc<dyn BusinessDayCalendarTrait>,
}

impl PartialEq for Following {
    fn eq(&self, other: &Self) -> bool {
        // Compare l'adresse des trait objects contenus dans le Box.
        ptr::eq(self.calendar.as_ref(), other.calendar.as_ref())
    }
}

impl Eq for Following {}

impl Following {
    /// Constructeur qui prend le calendrier en Box.
    pub fn new(calendar: Rc<dyn BusinessDayCalendarTrait>) -> Self {
        Following { calendar }
    }
}

impl BusinessDayConventionTrait for Following {
    /// Décale la date tant que celle-ci n'est pas ouvrée selon le calendrier.
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime {
        let mut shifted_date = *date;
        while !self.calendar.is_business_day(&shifted_date) {
            shifted_date += Duration::days(1);
        }
        shifted_date
    }
}
