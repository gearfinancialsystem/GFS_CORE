
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

pub trait TraitBusinessDayAdjuster {
    fn shift(&self, date: &PhantomIsoDatetimeW) -> PhantomIsoDatetimeW;
}