use crate::subtypes::Percentage::Percentage;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(Debug, PartialEq)]
pub struct CoverageOfCreditEnhancement(f64);

impl Default for CoverageOfCreditEnhancement {
    fn default() -> Self {
        CoverageOfCreditEnhancement(1.0)
    }
}

impl TraitTermDescription for CoverageOfCreditEnhancement {
    fn get_identifier(&self) -> &str {
        "coverageOfCreditEnhancement"
    }
    fn get_group(&self) -> &str {
        "Counterparty"
    }
    fn get_name(&self) -> &str {
        "Coverage Of Credit Enhancement"
    }
    fn get_acronym(&self) -> &str {
        "CECV"
    }
    fn get_type(&self) -> &str {
        "Real"
    }
    fn get_allowed_values(&self) -> &str {
        "['(0,1)']"
    }
    fn get_default_value(&self) -> &str {
        "1"
    }
    fn get_description(&self) -> &str {
        "Defines which percentage of the exposure is covered"
    }
}