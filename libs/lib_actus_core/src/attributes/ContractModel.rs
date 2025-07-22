use std::collections::HashMap;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_types::types::Value::Value;
use crate::contracts::Pam::PAM;
// use crate::contracts::Swaps::SWAPS;
use crate::contracts::Fxout::FXOUT;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitContractModel::TraitContractModel;



#[derive(Debug, Clone, PartialEq)]
pub enum ContractModel {
    PAM(PAM),
    // SWAPS(SWAPS),
    FXOUT(FXOUT)
}

impl ContractModel {
    
    pub fn new(sm_terms: &HashMap<String, Value>,
               risk_factors: &Option<RiskFactorModel>) -> Result<ContractModel, String> {
        let ct = sm_terms.get("contractType").unwrap().as_string().unwrap().as_str();
        match ct {
            "PAM" => {
                Ok(Self::PAM({
                    let mut c = PAM::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);


                    c
                }))

            }
            // "SWAPS" => {
            //     Ok(Self::SWAPS({
            //         let mut c = SWAPS::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            // 
            //         c
            //     }))
            // 
            // }
            "FXOUT" => {
                Ok(Self::FXOUT({
                    let mut c = FXOUT::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
            
                    c
                })) },
            _ => Err(format!("Unknown contract type {}", ct))
        }
    }
    
    pub fn run_schedule(&mut self, to: Option<IsoDatetime>) {
        match self {
            ContractModel::PAM(c) => {c.schedule(to)},
            // ContractModel::SWAPS(c) => {c.schedule(to)},
            ContractModel::FXOUT(c) => {c.schedule(to)},
        }
    }

    pub fn run_apply(&mut self, result_set_toogle: bool) {
        match self {
            ContractModel::PAM(c) => {c.apply(result_set_toogle)},
            // ContractModel::SWAPS(c) => {c.apply(result_set_toogle)},
            ContractModel::FXOUT(c) => {c.apply(result_set_toogle)},
        }
    }

    pub fn run(&mut self, to: Option<IsoDatetime>, result_set_toogle: bool) {
        self.run_schedule(to);
        self.run_apply(result_set_toogle);
    }

}