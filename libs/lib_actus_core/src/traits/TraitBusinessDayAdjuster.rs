use chrono::NaiveDateTime;

pub trait TraitBusinessDayAdjuster {
    fn shift(&self, date: &NaiveDateTime) -> NaiveDateTime;
}