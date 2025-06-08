use chrono::NaiveDateTime;

pub trait TraitBusinessDayConvention {
    fn shift(&self, date: &NaiveDateTime) -> NaiveDateTime;
}