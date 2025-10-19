
use std::{fmt, ptr};
use std::rc::Rc;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::traits::TraitBusinessDayCalendar::TraitBusinessDayCalendar;
use crate::traits::TraitBusinessDayAdjuster::TraitBusinessDayAdjuster;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;

#[derive(Clone, Debug)]
pub struct Following {
    pub calendar: Rc<Calendar>,
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
    pub fn new(calendar: Rc<Calendar>) -> Self {
        Following { calendar }
    }
}


impl TraitBusinessDayAdjuster for Following {
    /// Décale la date tant que celle-ci n'est pas ouvrée selon le calendrier.
    fn shift(&self, date: &PhantomIsoDatetimeW) -> PhantomIsoDatetimeW {
        let mut shifted_date = *date;
        while !self.calendar.is_business_day(&shifted_date) {
            shifted_date = shifted_date.add_period(PhantomIsoPeriodW::new(0,0, 1));
        }
        shifted_date
    }
}

impl fmt::Display for Following {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Following (Calendar : {})", self.calendar.to_string())
    }
}