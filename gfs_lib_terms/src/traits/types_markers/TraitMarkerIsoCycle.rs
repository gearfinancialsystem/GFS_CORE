use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;
use gfs_lib_types::types::IsoCycle::IsoCycle;
use crate::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;

pub trait TraitMarkerIsoCycle
where
    Self: Clone + Copy + Hash + Debug + Display + FromStr
{
    
    // Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Default
    fn value(&self) -> IsoCycle;

    fn set_value(&mut self, value: &IsoCycle);
    
    fn to_phantom_type(&self) -> PhantomIsoCycleW;
    
}