use std::collections::HashMap;
use crate::contracts::Pam::PAM;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::util::Value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum ContractModel {
    PAM(PAM),
    None
}

impl ContractModel {
    pub fn new(sm: &HashMap<String, Value>) -> ContractModel {
        let ct = sm.get("contractType").unwrap().as_string().unwrap().as_str();
        match ct {
            "PAM" => Self::PAM(PAM::new()),
            _ => Self::None
        }
    }
}