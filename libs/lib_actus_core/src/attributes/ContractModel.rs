use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::terms::grp_contract_identification::contract_types::Pam::PAM;
use crate::terms::grp_contract_identification::contract_types::Swaps::SWAPS;

#[derive(PartialEq, Debug, Clone)]
pub enum ContractModel {
    PAM(PAM),
    SWAPS(SWAPS),
}

impl ContractModel {
    pub fn new(sm: &HashMap<String, String>) -> Result<ContractModel, String> {
        let ct = sm.get("contractType").unwrap();
        match ct.as_str() {
            "PAM" => {
                let mut cm = PAM::init();
                cm.parse_from_dict(sm);
                Ok(ContractModel::PAM(cm))
            }
            "SWAPS" => {
                let mut cm = SWAPS::init();
                cm.parse_from_dict(sm);
                Ok(ContractModel::SWAPS(cm))
            }
            _ => Err("test erreur".to_string()),

        }
    }
}
//Implémentation de Deref pour ContractModel
impl Deref for ContractModel {
    type Target = ContractModel;

    fn deref(&self) -> &Self::Target {
        match self {
            ContractModel::PAM(pam) => pam,
            ContractModel::SWAPS(swaps) => swaps,
        }
    }
}

// Implémentation de DerefMut pour ContractModel
impl DerefMut for ContractModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ContractModel::PAM(pam) => pam,
            ContractModel::SWAPS(swaps) => swaps,
        }
    }
}