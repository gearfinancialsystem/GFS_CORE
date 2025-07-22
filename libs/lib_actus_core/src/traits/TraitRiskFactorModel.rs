
use std::collections::HashSet;
use crate::attributes::ContractTerms::ContractTerms;
use crate::events::ContractEvent::ContractEvent;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;

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
        states: &StateSpace,
        attributes: &ContractTerms,
        is_market: bool,
    ) -> Option<f64>;
}
