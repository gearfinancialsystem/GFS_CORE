use crate::attributes::ContractModel::ContractModel;
use crate::attributes::Dependence::Dependence;

#[derive(Debug, Clone)]
pub struct ContractElem {
    contract_elem: ContractModel,
    dependence: Dependence
}