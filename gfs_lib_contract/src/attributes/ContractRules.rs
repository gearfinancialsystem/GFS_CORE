use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::reference_type::ReferenceType::ReferenceType;

#[derive(Debug, Clone, PartialEq)]
pub enum ContractRules {
    SwapsRulesE(SwapsRules),
}

// Make the struct public
#[derive(Debug, Clone, PartialEq)]
pub struct SwapsRules {
    pub identifier_leg1: ContractID,
    pub reference_type_leg1: ReferenceType,
    pub reference_role_leg1: ReferenceRole,
    pub identifier_leg2: ContractID,
    pub reference_type_leg2: ReferenceType,
    pub reference_role_leg2: ReferenceRole,
}

impl Default for SwapsRules {
    fn default() -> Self {
        Self {
            identifier_leg1: ContractID::new("".to_string()).unwrap(),
            reference_type_leg1: ReferenceType::CNT,
            reference_role_leg1: ReferenceRole::COVI,
            identifier_leg2: ContractID::new("".to_string()).unwrap(),
            reference_type_leg2: ReferenceType::CNT,
            reference_role_leg2: ReferenceRole::COVI
        }
    }
}

impl SwapsRules {
    pub fn update_from(
        &mut self,
        identifier_leg1: String,
        reference_type_leg1: ReferenceType,
        reference_role_leg1: ReferenceRole,
        identifier_leg2: String,
        reference_type_leg2: ReferenceType,
        reference_role_leg2: ReferenceRole,
    ) -> Self {
        Self {
            identifier_leg1: ContractID::new(identifier_leg1).unwrap(),
            reference_type_leg1,
            reference_role_leg1,
            identifier_leg2: ContractID::new(identifier_leg2).unwrap(),
            reference_type_leg2,
            reference_role_leg2,
        }
    }

    pub fn create_from(
        identifier_leg1: String,
        reference_type_leg1: ReferenceType,
        reference_role_leg1: ReferenceRole,
        identifier_leg2: String,
        reference_type_leg2: ReferenceType,
        reference_role_leg2: ReferenceRole,
    ) -> Self {
        Self {
            identifier_leg1: ContractID::new(identifier_leg1).unwrap(),
            reference_type_leg1,
            reference_role_leg1,
            identifier_leg2: ContractID::new(identifier_leg2).unwrap(),
            reference_type_leg2,
            reference_role_leg2,
        }
    }
}

