use std::collections::HashSet;
use crate::events::ContractEvent::{ContractEvent, TraitContractEvent};


pub trait TraitExternalEvent<TfState, TfPayoff, Tdtime1, Tdtime2>: Copy + Clone
{
    /// Returns the set of unique risk factor IDs
    fn keys(&self) -> Option<HashSet<String>>;

    /// Returns the set of event times for a particular risk factor
    /// The default implementation returns an empty set of events.
    fn events(&self, contract_id: String) -> HashSet<ContractEvent>;
    
}
