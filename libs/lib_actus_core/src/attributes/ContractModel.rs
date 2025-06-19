use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::terms::grp_contract_identification::contract_types::Pam::PAM;

#[derive(PartialEq, Debug, Clone)]
pub enum ContractModel {
    PAM(PAM),
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
            _ => Err("test erreur".to_string()),

        }
    }
}
// Implémentation de Deref pour ContractModel
impl Deref for ContractModel {
    type Target = PAM;

    fn deref(&self) -> &Self::Target {
        match self {
            ContractModel::PAM(pam) => pam,
        }
    }
}

// Implémentation de DerefMut pour ContractModel
impl DerefMut for ContractModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ContractModel::PAM(pam) => pam,
        }
    }
}