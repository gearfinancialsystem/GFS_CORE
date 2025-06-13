use std::collections::HashMap;
use crate::terms::grp_contract_identification::contract_types::Pam::PAM;
use crate::traits::TraitContractModel::TraitContractModel;

pub struct ContractModel {
    pub cm: dyn TraitContractModel,
}

impl ContractModel {
    pub fn new(sm: &HashMap<String, String>) -> Result<ContractModel, String> {
        let ct = sm.get("ContractType").unwrap();
        match ct.as_str() {
            "PAM" => {
                let mut cm = PAM::default();
                cm.parse_from_dict(sm);
                Ok(ContractModel {cm: cm})
            }
            _ => Err("test erreur".to_string()),

            }
        }
    }
}