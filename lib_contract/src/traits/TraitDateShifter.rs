use chrono::NaiveDateTime;

pub trait TraitDateShifter {
    fn shift(&self, date: NaiveDateTime) -> NaiveDateTime;
}