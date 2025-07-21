use std::collections::HashMap;
use crate::contracts::Pam::PAM;
use crate::contracts::Swaps::SWAPS;
use crate::contracts::Fxout::FXOUT;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::Value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum ContractModel {
    PAM(PAM),
    SWAPS(SWAPS),
    FXOUT(FXOUT)
}

impl ContractModel {
    
    pub fn new(sm: &HashMap<String, Value>) -> Result<ContractModel, String> {
        let ct = sm.get("contractType").unwrap().as_string().unwrap().as_str();
        match ct {
            "PAM" => {
                Ok(Self::PAM({
                    let mut c = PAM::new();
                    c.set_contract_terms(sm);
                    c.set_contract_risk_factors(sm);
                    c.set_contract_structure(sm);

                    c
                }))

            }
            "SWAPS" => {
                Ok(Self::SWAPS({
                    let mut c = SWAPS::new();
                    c.set_contract_terms(sm);
                    c.set_contract_risk_factors(sm);
                    c.set_contract_structure(sm);

                    c
                }))

            }
            "FXOUT" => {
                Ok(Self::FXOUT({
                    let mut c = FXOUT::new();
                    c.set_contract_terms(sm);
                    c.set_contract_risk_factors(sm);
                    c.set_contract_structure(sm);

                    c
                }))

            }
            _ => Err(format!("Unknown contract type {}", ct))
        }
    }
    
    pub fn run_schedule(&mut self, to: Option<IsoDatetime>) {
        match self {
            ContractModel::PAM(c) => {c.schedule(to)},
            ContractModel::SWAPS(c) => {c.schedule(to)},
            ContractModel::FXOUT(c) => {c.schedule(to)},
        }
    }

    pub fn run_apply(&mut self, result_set_toogle: bool) {
        match self {
            ContractModel::PAM(c) => {c.apply(result_set_toogle)},
            ContractModel::SWAPS(c) => {c.apply(result_set_toogle)},
            ContractModel::FXOUT(c) => {c.apply(result_set_toogle)},
        }
    }

    pub fn run(&mut self, to: Option<IsoDatetime>, result_set_toogle: bool) {
        self.run_schedule(to);
        self.run_apply(result_set_toogle);
    }

}