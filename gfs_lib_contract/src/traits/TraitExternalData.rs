
use std::collections::HashSet;
use std::fmt::Debug;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;


pub trait TraitExternalData
{
    /// Returns the set of unique risk factor IDs
    fn keys(&self) -> Option<HashSet<String>>;

    /// Returns the state of a particular risk factor at a future time
    fn state_at(
        &self,
        id: String,
        time: &PhantomIsoDatetimeW
    ) -> Option<f64>;
}
