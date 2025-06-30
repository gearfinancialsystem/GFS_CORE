use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::state_space::StateSpace::StateSpace;
use crate::types::IsoDatetime::IsoDatetime;

pub struct RiskFactorModel;

impl RiskFactorModel {
    pub fn events(&self, attributes: &ContractModel) -> Vec<ContractEvent> {
        let events = Vec::new();
        events
    }
    
    pub fn state_at(&self, id: &String, time: &IsoDatetime, states: &StateSpace, attributes: &ContractModel, isMarket: bool) -> Option<f64> {
        Some(1.0) // a implementer
    }
}