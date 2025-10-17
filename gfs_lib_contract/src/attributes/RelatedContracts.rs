use std::collections::{HashMap, HashSet};
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use crate::attributes::ContractElem::ContractElem;
use crate::attributes::ContractRules::ContractRules;

#[derive(Debug, Clone)]
pub struct RelatedContracts {
    pub contract_set: HashMap<ContractID, ContractElem>,
    pub contract_structure : Option<ContractRules>
}

