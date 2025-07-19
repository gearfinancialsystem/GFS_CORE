use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::str::FromStr;
use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventSequence::EventSequence;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::events::EventType::EventType;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::util_tests::essai_load_data_observed::ObservedDataSet;
use crate::util_tests::essai_load_data_observed::ObservedDataPoint;
use crate::util_tests::essai_load_event_observed::ObservedEvent;

pub struct DataObserver {
    multi_series: HashMap<String, HashMap<IsoDatetime, f64>>,
    events_observed: HashMap<String, Vec<ContractEvent<IsoDatetime, IsoDatetime>>>,
}

impl DataObserver {
    pub fn new() -> Self {
        Self {
            multi_series: HashMap::new(),
            events_observed: HashMap::new(),
        }
    }



    pub fn add(&mut self, symbol: String, series: HashMap<IsoDatetime, f64>) {
        self.multi_series.insert(symbol, series);
    }

    pub fn set_events_observed(&mut self, observed_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>) {
        for event in observed_events {
            if let Some(contract_id) = &event.contract_id {
                let id = contract_id.clone();
                self.events_observed.entry(id.value())
                    .or_insert_with(Vec::new)
                    .push(event);
            }
        }
    }

}
impl TraitRiskFactorModel for DataObserver {

    fn keys(&self) -> HashSet<String> {
        self.multi_series.keys().cloned().collect()
    }

    fn events(&self, _model: &ContractModel) // &impl TraitContractModel
        -> HashSet<ContractEvent<IsoDatetime, IsoDatetime>> {
        self.events_observed
            .values()
            .flat_map(|list| list.iter().cloned())
            .collect()
    }

    fn state_at(
        &self,
        id: String,
        time: &IsoDatetime,
        _contract_states: &StateSpace,
        _contract_attributes: &ContractModel,
        _is_market: bool
    ) -> f64 {
        self.multi_series
            .get(id.as_str())
            .and_then(|series| series.get(time))
            .copied()
            .unwrap_or(0.0)
    }
}


// Fonction pour créer un DataObserver à partir des données chargées
pub fn create_observer(
    data_observed: HashMap<String, ObservedDataSet>,
    events_observed: Vec<ObservedEvent>,
    currency: &str
) -> DataObserver {
    let mut observer = DataObserver::new();

    // Ajouter les séries de données observées
    for (symbol, dataset) in data_observed {
        let mut series = HashMap::new();
        for point in dataset.get_data() {
            if let Ok(timestamp) = IsoDatetime::from_str(&point.get_timestamp()) {
                series.insert(timestamp, point.get_value());
            }
        }
        observer.add(symbol, series);
    }

    // Convertir et ajouter les événements observés
    let contract_events = convert_observed_events(events_observed, currency);
    observer.set_events_observed(contract_events.expect("correctness"));

    observer
}

// Conversion des événements observés
fn convert_observed_events(
    events: Vec<ObservedEvent>,
    currency: &str
) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, Box<dyn std::error::Error>> {
    events.into_iter().map(|obs_event| {
        // Convertir les temps
        let event_time = IsoDatetime::from_str(&obs_event.get_time())?;
        let schedule_time = event_time.clone(); // Même temps pour schedule_time

        // Convertir le type d'événement
        let event_type = EventType::from_str(&obs_event.get_typex())
            .map_err(|_| format!("Unknown event type: {}", obs_event.get_typex()))?;

        // Créer les wrappers nécessaires
        let currency_wrapper = Currency::new(currency.to_string())
            .map_err(|_| format!("Invalid currency: {}", currency))?;

        let contract_id_wrapper = ContractID::new(obs_event.get_contract_id())
            .map_err(|_| format!("Invalid contract ID: {}", obs_event.get_contract_id()))?;

        // Calculer l'epoch offset
        let epoch_offset = event_time.0.and_utc().timestamp_millis() +
            EventSequence::time_offset(&event_type);

        // Créer l'événement de contrat
        let mut event = ContractEvent {
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
            epoch_offset: Some(epoch_offset),
            fstate: None,
            fpayoff: None,
            event_time: Some(event_time),
            schedule_time: Some(schedule_time),
            event_type,
            currency: Some(currency_wrapper),
            payoff: Some(obs_event.get_value()),
            state: obs_event.get_states().clone(),
            contract_id: Some(contract_id_wrapper),
        };

        Ok(event)
    }).collect()
}