use crate::attributes::ContractModel::ContractModel;
use crate::contracts::PrincipalAtMaturity::PrincipalAtMaturity;
use crate::contracts::Stock::Stock;
use crate::contracts::Swap::Swap;
use crate::types::isoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;
use crate::externals::RiskFactorModel::RiskFactorModel;

#[derive(Debug, PartialEq)]
pub struct ContractType;

impl ContractType {


    pub fn schedule(to: Option<IsoDatetime>, cm: &ContractModel) -> Option<Vec<ContractEvent>> {

        match cm.clone().contractType.unwrap().as_str() {
            "PAM" => Some(PrincipalAtMaturity::schedule(&to.unwrap(), cm).unwrap()),
            "SWAPS" => Some(Swap::schedule(&to.unwrap(),cm).unwrap()),
            "STK" => Some(Stock::schedule(&to.unwrap(),cm).unwrap()),
            _ => None
        }

    }
    pub fn apply(events: Vec<ContractEvent>, cm: &ContractModel, observer: &RiskFactorModel) -> Option<Vec<ContractEvent>> {

        match cm.clone().contractType.unwrap().as_str() {
            "PAM" => Some(PrincipalAtMaturity::apply(events, cm, observer)),
            "SWAPS" => Some(Swap::apply(events, cm, observer)),
            "STK" => Some(Stock::apply(events, cm, observer)),
            _ => None
        }
    }
}




