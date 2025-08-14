
use std::collections::HashSet;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::events::ContractEvent::{ContractEvent, TraitContractEvent};


pub trait TraitRiskFactorModel<CE>: Copy + Clone
where
    CE: TraitContractEvent,
{
    /// Returns the set of unique risk factor IDs
    fn keys(&self) -> Option<HashSet<String>>;

    /// Returns the set of event times for a particular risk factor
    /// The default implementation returns an empty set of events.
    fn events(&self, contract_id: String) -> HashSet<CE>;

    /// Returns the state of a particular risk factor at a future time
    fn state_at(
        &self,
        id: String,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        attributes: &ContractTerms,
        is_market: bool,
    ) -> Option<f64>;
}
