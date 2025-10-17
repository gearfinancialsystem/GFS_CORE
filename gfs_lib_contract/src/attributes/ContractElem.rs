use crate::attributes::ContractModel::ContractModel;
use crate::attributes::Dependence::Dependence;

#[derive(Debug, Clone)]
pub struct ContractElem {
    pub contract_elem: ContractModel,
    pub dependence: Dependence
}