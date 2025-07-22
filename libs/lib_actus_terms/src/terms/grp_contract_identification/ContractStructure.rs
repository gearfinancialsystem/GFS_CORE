use crate::attributes::ContractReference::ContractReference;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractStructure(pub Vec<ContractReference>);

impl ContractStructure {
    pub fn new(contract_structure: Vec<ContractReference>) -> Self {
        ContractStructure(contract_structure)
    }
}