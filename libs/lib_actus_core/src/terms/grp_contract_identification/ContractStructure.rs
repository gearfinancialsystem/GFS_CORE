
use crate::contract_reference::ContractReference::ContractReference;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, Eq, PartialEq)]
pub struct ContractStructure {
    value: Option<Vec<ContractReference>>
}

impl Default for ContractStructure {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl TraitTermDescription for ContractStructure {
    fn get_identifier(&self) -> &str {
        "contractStructure"
    }
    fn get_group(&self) -> &str {
        "Contract identification"
    }
    fn get_name(&self) -> &str {
        "Contract Structure"
    }
    fn get_acronym(&self) -> &str {
        "CTS"
    }
    fn get_type(&self) -> &str {
        "ContractReference[]"
    }
    fn get_allowed_values(&self) -> &str {
        "[]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "A structure identifying individual or sets of underlying contracts. E.g. for FUTUR, this structure identifies the single underlying contract, for SWAPS, the FirstLeg and SecondLeg are identified, or for CEG, CEC the structure identifies Covered and Covering contracts."
    }
}    