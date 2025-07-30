use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;
use lib_actus_types::types::IsoCycle::IsoCycle;

pub trait TraitMarkerIsoCycle
where
    Self: Clone + Copy + Hash + Debug + Display + FromStr
{
    fn value(&self) -> IsoCycle;

    fn set_value(&mut self, value: &IsoCycle);
}