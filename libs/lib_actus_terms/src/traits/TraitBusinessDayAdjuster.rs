use chrono::NaiveDateTime;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

pub trait TraitBusinessDayAdjuster {
    fn shift(&self, date: &IsoDatetime) -> IsoDatetime;
}