// l'objectif est d'utiliser des 'extension traits' pour NaiveDateTime

// En Rust, vous pouvez créer un alias de type en utilisant le mot-clé type. Cependant,
// vous ne pouvez pas directement implémenter un trait pour un alias de type. Les traits
// doivent être implémentés pour un type concret, et un alias de type n'est pas considéré
// comme un type distinct du type original.

use std::ops::Add;
use std::ops::Sub;
use chrono::{Days, Months, NaiveDateTime, NaiveDate, Datelike};
use crate::types::IsoPeriod::IsoPeriod;


// Définition d'un alias de type
pub type IsoDatetime = NaiveDateTime;

// Implémentation d'un trait pour le type original (u32 dans ce cas)

pub trait traitNaiveDateTimeExtension {
    fn double(&self) -> Self;
    fn is_last_day_of_month(&self) -> bool;
}

impl traitNaiveDateTimeExtension for NaiveDateTime {
    fn double(&self) -> Self {
        *self
    }
    fn is_last_day_of_month(&self) -> bool {
        // Add one day to the given date
        let first_day_of_next_month = {
            if self.date().month() == 12 {
                NaiveDate::from_ymd_opt((self.date().year() + 1) as i32, 1, 1).unwrap()
            }
            else {
                NaiveDate::from_ymd_opt(self.date().year() as i32, self.date().month() + 1, 1).unwrap()
            }
        };
        let last_day_of_month = first_day_of_next_month.pred().day();
        if last_day_of_month == self.day() {
            true
        }
        else {
            false
        }

    }
}





impl Add<IsoPeriod> for IsoDatetime {
    type Output = IsoDatetime;

    fn add(self, other: IsoPeriod) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        self.checked_add_days(Days::new(other.days as u64)).unwrap()
            .checked_add_months(Months::new(other.months as u32)).unwrap()
            .checked_add_months(Months::new((other.years * 12) as u32)).unwrap()
    }
}

impl Sub<IsoPeriod> for IsoDatetime {
    type Output = IsoDatetime;

    fn sub(self, other: IsoPeriod) -> IsoDatetime {
        // Convert Yards to Meters and add to self
        self.checked_sub_days(Days::new(other.days as u64)).unwrap()
            .checked_sub_months(Months::new(other.months as u32)).unwrap()
            .checked_sub_months(Months::new((other.years * 12) as u32)).unwrap()
    }
}