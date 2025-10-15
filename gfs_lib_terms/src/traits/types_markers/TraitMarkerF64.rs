use std::fmt::{Debug, Display};
use std::str::FromStr;
use crate::phantom_terms::PhantomF64::PhantomF64W;

pub trait TraitMarkerF64
where
    Self: Clone + Copy + Debug + Display + FromStr
{

    fn value(&self) -> f64;
    
    //fn to_phantom_type(&self) -> PhantomF64W;
}