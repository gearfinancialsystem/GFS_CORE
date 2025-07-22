use std::collections::{HashMap, HashSet};
use lib_actus_core::attributes::ContractTerms::ContractTerms;
use lib_actus_core::events::ContractEvent::ContractEvent;
use lib_actus_core::state_space::StateSpace::StateSpace;
use lib_actus_core::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_core::types::IsoDatetime::IsoDatetime;
use crate::risk_factors::risk_factor_model_1::DataObserved::DataObserved;
use crate::risk_factors::risk_factor_model_1::EventObserved::EventObserved;


#[derive(Debug, Clone, PartialEq)]
pub struct RiskFactorModel1 {
    data_observed: Option<DataObserved>,
    events_observed: Option<EventObserved>,
}

impl RiskFactorModel1 {
    pub fn new() -> Self {
        Self {
            data_observed: None,
            events_observed: None,
        }
    }

    pub fn new_from(file_path: &str,
                    test_case_id: &str) -> RiskFactorModel1 {
        Self {
            data_observed: DataObserved::new_from(file_path, test_case_id).ok(),
            events_observed: EventObserved::new_from(file_path, test_case_id).ok(),
        }
    }


}
impl TraitRiskFactorModel for RiskFactorModel1 {

    // chope les cles de data_series dans DataObserver
    fn keys(&self) -> Option<HashSet<String>> {
        if let Some( ms ) = &self.data_observed {
            Some(ms.data_series.keys().cloned().collect())
        } else {
            None
        }
    }

    fn events(&self, contract_id: String) // &impl TraitContractModel
              -> HashSet<ContractEvent<IsoDatetime, IsoDatetime>> {
        if let Some(eo) = &self.events_observed {
            let a = eo.event_serie.get(contract_id.as_str()).cloned();
            if let Some(eo) = a {
                let set: HashSet<_> = eo.into_iter().collect();
                set
            }
            else {
                return HashSet::new()
            }
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
            let ds = ms.data_series.get(id.as_str())?;
            let point = ds.get_data_point_at_specific_timestamps(time);

            if let Some(point) = point {
                Some(point.value)
            }
            else { 
                None
            }
        }
        else {
            None
        }
    }
}


// Fonction pour créer un DataObserver à partir des données chargées
// pub fn create_risk_factor(
//     data_observed: HashMap<String, DataObserved>,
//     events_observed: Vec<EventObserver>,
//     currency: &str
// ) -> RiskFactors {
//     let mut observer = RiskFactors::new();
// 
//     // Ajouter les séries de données observées
//     for (symbol, dataset) in data_observed {
//         let mut series = HashMap::new();
//         for point in dataset.get_data() {
//             if let timestamp = point.timestamp {
//                 series.insert(timestamp, point.get_value());
//             }
//         }
//         observer.add_data_observed_item(symbol, series);
//     }
// 
//     // Convertir et ajouter les événements observés
//     let contract_events = convert_observed_events(events_observed, currency);
//     observer.set_events_observed(contract_events.expect("correctness"));
// 
//     observer
// }

// Conversion des événements observés
