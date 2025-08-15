use std::collections::HashSet;
use crate::attributes::ContractElem::ContractElem;
use crate::attributes::ContractRules::ContractRules;

#[derive(Debug, Clone)]
pub struct RelatedContracts {
    pub ContractSet: HashSet<ContractElem>,
    pub ContractStructure : Option<ContractRules>
}

