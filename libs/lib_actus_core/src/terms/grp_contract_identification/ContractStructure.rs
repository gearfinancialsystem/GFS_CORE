use std::collections::HashMap;
use crate::attributes::ContractReference::ContractReference;
use crate::util::Value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractStructure(Vec<ContractReference>);

impl ContractStructure {
    pub fn new(contract_structure: Vec<ContractReference>) -> Self {
        ContractStructure(contract_structure)
    }
}