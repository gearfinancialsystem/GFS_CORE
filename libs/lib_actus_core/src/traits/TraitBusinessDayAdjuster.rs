use chrono::NaiveDateTime;
use crate::types::IsoDatetime::IsoDatetime;

pub trait TraitBusinessDayAdjuster {
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime;
}