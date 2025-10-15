use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;
use gfs_lib_types::types::IsoPeriod::IsoPeriod;
use crate::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;

pub trait TraitMarkerIsoPeriod
where
    Self: Clone + Copy + Hash + Debug + Display + FromStr
{

    fn value(&self) -> IsoPeriod;

    fn set_value(&mut self, value: &IsoPeriod);

    fn parse_from_string(s: &str) -> Result<IsoPeriod, String>;

    //fn to_phantom_type(&self) -> PhantomIsoPeriodW;
}