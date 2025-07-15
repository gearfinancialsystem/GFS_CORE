use std::collections::{HashMap, HashSet};
use crate::events::ContractEvent::ContractEvent;

use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;

pub struct DataObserver {
    multi_series: HashMap<String, HashMap<IsoDatetime, f64>>,
    events_observed: HashMap<String, Vec<ContractEvent<IsoDatetime, IsoDatetime>>>,
}

impl DataObserver {
    pub fn new() -> Self {
        Self {
            multi_series: HashMap::<String, HashMap<IsoDatetime, f64>>::new(),
            events_observed: HashMap::<String, Vec<ContractEvent<IsoDatetime, IsoDatetime>>>::new()
        }
    }
    pub fn keys(&self) -> HashSet<String> {
        self.multi_series.keys().cloned().collect::<HashSet<String>>()
    }

    pub fn add(&mut self, symbol: String, series: HashMap<IsoDatetime, f64>) {
        &self.multi_series.insert(symbol, series);
    }
    pub fn set_events_observed(&mut self, observed_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> ) {
        let s = observed_events.get(0).cloned().unwrap().contract_id.unwrap().value().to_string();
        &self.events_observed.insert(s, observed_events);
    }

    pub fn state_at(&self,
                    id: String,
                    time: IsoDatetime,
                    contract_states: StateSpace,
                    contract_attributes: impl TraitContractModel,
                    is_market: bool) -> f64 {
        let a = &self.multi_series[&id][&time];
        a.clone()
    // A revoir
    }

    pub fn events(&self, model: impl TraitContractModel) -> HashSet<ContractEvent<IsoDatetime, IsoDatetime>>{
        self.events_observed.values()
            .flat_map(|list| list.iter().cloned()) // Aplatit chaque Vec en ses éléments et clone les éléments
            .collect()
        //return eventsObserved.values().stream().flatMap(list->list.stream()).collect(Collectors.toSet());
    }
}
