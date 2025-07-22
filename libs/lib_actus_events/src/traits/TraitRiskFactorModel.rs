
use std::collections::HashSet;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;


pub trait TraitRiskFactorModel {
    /// Returns the set of unique risk factor IDs
    fn keys(&self) -> Option<HashSet<String>>;

    /// Returns the set of event times for a particular risk factor
    ///
    /// The default implementation returns an empty set of events.
    fn events(&self, contract_id: String) -> HashSet<ContractEvent<IsoDatetime, IsoDatetime>>;

    /// Returns the state of a particular risk factor at a future time
    fn state_at(
        &self,
        id: String,
        time: &IsoDatetime,
        states: &StatesSpace,
        attributes: &ContractTerms,
        is_market: bool,
    ) -> Option<f64>;
}
