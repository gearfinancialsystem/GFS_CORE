use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;

pub trait TraitMarkerIsoDatetime
where
    Self: Clone + Copy + Hash + Debug + Display + FromStr + From<IsoDatetime>
{

    fn value(&self) -> IsoDatetime;

    fn set_value(&mut self, value: &IsoDatetime);

    fn parse_from_string(s: &str, fmt: &str) -> Result<IsoDatetime, String>;

    fn to_phantom_type(&self) -> PhantomIsoDatetimeW;
}