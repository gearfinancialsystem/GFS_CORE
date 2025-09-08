use std::collections::HashSet;
use crate::attributes::ContractElem::ContractElem;
use crate::attributes::ContractRules::ContractRules;

#[derive(Debug, Clone)]
pub struct RelatedContracts {
    pub contract_set: HashSet<ContractElem>,
    pub contract_structure : Option<ContractRules>
}

