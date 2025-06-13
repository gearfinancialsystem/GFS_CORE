// l'objectif est d'utiliser des 'extension traits' pour NaiveDateTime

// En Rust, vous pouvez créer un alias de type en utilisant le mot-clé type. Cependant,
// vous ne pouvez pas directement implémenter un trait pour un alias de type. Les traits
// doivent être implémentés pour un type concret, et un alias de type n'est pas considéré
// comme un type distinct du type original.

use std::ops::Add;
use std::ops::Sub;
use chrono::{Days, Months, NaiveDateTime};
use crate::types::IsoPeriod::IsoPeriod;

// Définition d'un alias de type
pub type IsoDatetime = NaiveDateTime;

// Implémentation d'un trait pour le type original (u32 dans ce cas)
trait traitNaiveDateTimeExtension {
    fn double(&self) -> Self;
}

impl traitNaiveDateTimeExtension for NaiveDateTime {
    fn double(&self) -> Self {
        *self
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