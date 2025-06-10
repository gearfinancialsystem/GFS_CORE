use std::error::Error;
use crate::contracts::ContractModel::ContractModel;
use crate::event::ContractEvent::ContractEvent;
use crate::external::RiskFactorModel::RiskFactorModel;
use chrono::NaiveDateTime;
use crate::traits::TraitEnumOptionDescription::TraitEnumOptionDescription;


// use crate::contracts::PrincipalAtMaturity::PrincipalAtMaturity;

#[derive(Debug, Eq, PartialEq)]
pub struct PAM;

impl PAM {
    pub fn new() -> Self {
        return PAM;
    }
    pub fn type_str(&self) -> String {
        return "PAM contract cont_type".to_string();
    }
}

// impl PAM {
//     pub fn schedule(to: NaiveDateTime, model: &ContractModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
//         PrincipalAtMaturity::schedule(to, model)
//     }
// 
//     /// Applies a set of contract events to the current state of the contract
//     pub fn apply(events: &mut Vec<ContractEvent>, model: &ContractModel, observer: &RiskFactorModel) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
//         Ok(PrincipalAtMaturity::apply(events, model, observer))
//     }
// }


