use chrono::NaiveDateTime;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

pub trait TraitBusinessDayAdjuster {
    fn shift(&self, date: &PhantomIsoDatetimeW) -> PhantomIsoDatetimeW;
}