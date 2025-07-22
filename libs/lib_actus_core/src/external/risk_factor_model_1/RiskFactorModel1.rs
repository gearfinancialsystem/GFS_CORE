use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::attributes::ContractTerms::ContractTerms;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventSequence::EventSequence;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::events::EventType::EventType;
use crate::external::data_observers::DataObserver1::DataObserver1;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

use crate::external::EventObserved::EventObserved;
use crate::external::DataObserved::DataObserved;

#[derive(Debug, Clone, PartialEq)]
pub struct RiskFactors {
    data_observed: Option<DataObserved>,
    events_observed: Option<EventObserved>,
}

impl RiskFactors {
    pub fn new() -> Self {
        Self {
            data_observed: None,
            events_observed: None,
        }
    }

    pub fn new_from(&self,
                    file_path: &str,
                    test_case_id: &str) -> DataObserved {
        Self {
            data_observed: Dat
        }
    }

    pub fn add_data_observed_item(&mut self, symbol: String, series: HashMap<IsoDatetime, f64>) {
        if let Some( ms ) = &mut self.data_observed{
            ms.insert(symbol, series);
        }
    }

    pub fn set_events_observed(&mut self, observed_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>) {
        for event in observed_events {
            if let Some(contract_id) = &event.contract_id {
                let id = contract_id.clone();
                if let Some(eo) = &mut self.events_observed{
                    eo.entry(id.value())
                        .or_insert_with(Vec::new)
                        .push(event);
                }
            }
        }
    }

}
impl TraitRiskFactorModel for RiskFactors {

    fn keys(&self) -> Option<HashSet<String>> {
        if let Some( ms ) = &self.data_observed {
            Some(ms.keys().cloned().collect())
        } else {
            None
        }
        //self.multi_series.keys().cloned().collect()
    }

    fn events(&self, _model: &ContractTerms) // &impl TraitContractModel
              -> HashSet<ContractEvent<IsoDatetime, IsoDatetime>> {
        if let Some(eo) = &self.events_observed {
            eo.values()
                .flat_map(|list| list.iter().cloned())
                .collect()
        } else {
            HashSet::new()
        }

    }

    fn state_at(
        &self,
        id: String,
        time: &IsoDatetime,
        _contract_states: &StateSpace,
        _contract_attributes: &ContractTerms,
        _is_market: bool
    ) -> Option<f64> {
        if let Some( ms ) = &self.data_observed {
            Some(ms.get(id.as_str())
                .and_then(|series| series.get(time))
                .copied()
                .unwrap_or(0.0))
        } else { None }
        // self.multi_series
        //     .get(id.as_str())
        //     .and_then(|series| series.get(time))
        //     .copied()
        //     .unwrap_or(0.0)
    }
}


// Fonction pour créer un DataObserver à partir des données chargées
pub fn create_risk_factor(
    data_observed: HashMap<String, DataObserved>,
    events_observed: Vec<EventObserver>,
    currency: &str
) -> RiskFactors {
    let mut observer = RiskFactors::new();

    // Ajouter les séries de données observées
    for (symbol, dataset) in data_observed {
        let mut series = HashMap::new();
        for point in dataset.get_data() {
            if let timestamp = point.timestamp {
                series.insert(timestamp, point.get_value());
            }
        }
        observer.add_data_observed_item(symbol, series);
    }

    // Convertir et ajouter les événements observés
    let contract_events = convert_observed_events(events_observed, currency);
    observer.set_events_observed(contract_events.expect("correctness"));

    observer
}

// Conversion des événements observés
