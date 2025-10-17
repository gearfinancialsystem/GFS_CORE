use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::reference_type::ReferenceType::ReferenceType;

#[derive(Debug, Clone, PartialEq)]
pub enum ContractRules {
    SwapsRules(SwapsRules),
}

impl ContractRules {
    pub fn new(contract_rules: ContractRules) -> Self {
        contract_rules
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwapsRules {
    pub identifier_leg1: String,
    pub reference_type_leg1: ReferenceType,
    pub reference_role_leg1: ReferenceRole,
    pub identifier_leg2: String,
    pub reference_type_leg2: ReferenceType,
    pub reference_role_leg2: ReferenceRole,
}

impl SwapsRules {
    pub fn create_from(
        identifier_leg1: String,
        reference_type_leg1: ReferenceType,
        reference_role_leg1: ReferenceRole,
        identifier_leg2: String,
        reference_type_leg2: ReferenceType,
        reference_role_leg2: ReferenceRole,
    ) -> Self {
        Self {
            identifier_leg1,
            reference_type_leg1,
            reference_role_leg1, // Correction du nom du champ
            identifier_leg2,
            reference_type_leg2,
            reference_role_leg2, // Correction du nom du champ
        }
    }
}

impl Default for SwapsRules {
    fn default() -> Self {
        Self {
            identifier_leg1: "".to_string(),
            reference_type_leg1:  ReferenceType::CNT,
            reference_role_leg1: ReferenceRole::COVI,
            identifier_leg2: "".to_string(),
            reference_type_leg2: ReferenceType::CNT,
            reference_role_leg2: ReferenceRole::COVI
        }
    }
}
