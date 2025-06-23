use crate::attributes::ContractReference::ContractReference;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractStructure(Vec<ContractReference>);
