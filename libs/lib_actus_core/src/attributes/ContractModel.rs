use std::collections::HashMap;
use crate::contracts::Pam::PAM;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::Value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum ContractModel {
    PAM(PAM)
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
            _ => Err(format!("Unknown contract type {}", ct))
        }
    }

    pub fn run_schedule(&mut self, to: Option<IsoDatetime>) {
        match self {
            ContractModel::PAM(c) => {c.schedule(to)}
        }
    }

    pub fn run_apply(&mut self, result_set_toogle: bool) {
        match self {
            ContractModel::PAM(c) => {c.apply(result_set_toogle)}
        }
    }

    pub fn run(&mut self, to: Option<IsoDatetime>, result_set_toogle: bool) {
        self.run_schedule(to);
        self.run_apply(result_set_toogle);
    }

}