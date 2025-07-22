use std::collections::HashSet;
use crate::attributes::ContractTerms::ContractTerms;
use crate::events::ContractEvent::ContractEvent;
use crate::external::risk_factor_model_1::RiskFactorModel1::RiskFactorModel1;



use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(Debug, Clone, PartialEq)]
pub enum RiskFactorModel {
    RiskFactorModel1(RiskFactorModel1)
}
impl TraitRiskFactorModel for RiskFactorModel {
    fn keys(&self) -> Option<HashSet<String>> {
        match self {
            RiskFactorModel::RiskFactorModel1(v) => {v.keys()}
        }
    }

    fn events(&self, contract_id: String) -> HashSet<ContractEvent<IsoDatetime, IsoDatetime>> {
        match self {
            RiskFactorModel::RiskFactorModel1(v) => {v.events(contract_id.clone())}
        }
    }

    fn state_at(&self, id: String, time: &IsoDatetime, states: &StateSpace, attributes: &ContractTerms, is_market: bool) -> Option<f64> {
        match self {
            RiskFactorModel::RiskFactorModel1(v) => {v.state_at(id, time, states, attributes, is_market)}
        }
    }
}

impl RiskFactorModel {

}