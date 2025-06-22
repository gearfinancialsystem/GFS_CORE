use std::collections::HashMap;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::reference_type::ReferenceType::ReferenceType;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::attributes::ContractModel::Value;

// Structure pour représenter ContractReference
pub struct ContractReference {
    pub reference_role: ReferenceRole,
    pub reference_type: ReferenceType,
    pub object: Object,
}

impl ContractReference {
    fn new(attributes: &HashMap<String, Value>, contract_role: &ContractRole) -> Self {
        let reference_role = ReferenceRole::from_str(attributes.get("referenceRole").unwrap().as_string().unwrap()).unwrap();
        let reference_type = ReferenceType::from_str(attributes.get("referenceType").unwrap().as_string().unwrap()).unwrap();
        let object = match reference_type {
            ReferenceType::CNT => {
                let mut child_model = attributes.get("object").unwrap().as_map().unwrap().clone();
                match (contract_role, &reference_role) {
                    (ContractRole::RFL, ReferenceRole::FIL) => {
                        child_model.insert("contractRole".to_string(), Object::String("RPA".to_string()));
                    },
                    (ContractRole::RFL, _) => {
                        child_model.insert("contractRole".to_string(), Object::String("RPL".to_string()));
                    },
                    (_, ReferenceRole::FIL) => {
                        child_model.insert("contractRole".to_string(), Object::String("RPL".to_string()));
                    },
                    (_, _) => {
                        child_model.insert("contractRole".to_string(), Object::String("RPA".to_string()));
                    }
                }
                Object::ContractModel(ContractModel::parse(&child_model))
            },
            ReferenceType::CID => {
                Object::String(attributes.get("object").unwrap().as_map().unwrap().get("contractIdentifier").unwrap().as_string().unwrap().clone())
            },
            ReferenceType::MOC => {
                Object::String(attributes.get("object").unwrap().as_map().unwrap().get("marketObjectCode").unwrap().as_string().unwrap().clone())
            },
            ReferenceType::EID => {
                attributes.get("object").unwrap().clone()
            },
            ReferenceType::CST => {
                Object::None
            }
        };

        ContractReference { reference_role, reference_type, object }
    }

    fn get_object(&self) -> &Object {
        &self.object
    }

    fn get_contract_attribute(&self, contract_attribute: &str) -> Option<String> {
        match &self.object {
            Object::String(s) if contract_attribute == "marketObjectCode" && matches!(self.reference_type, ReferenceType::MOC) => Some(s.clone()),
            Object::ContractModel(model) if matches!(self.reference_type, ReferenceType::CNT) => Some(model.get_as(contract_attribute).to_string()),
            Object::String(s) if matches!(self.reference_type, ReferenceType::CID) => Some(s.clone()),
            _ => None,
        }
    }

    // Ajoutez d'autres méthodes si nécessaire
}

// Énumération pour représenter Object
#[derive(Debug, Clone)]
enum Object {
    String(String),
    ContractModel(ContractModel),
    Map(HashMap<String, Object>),
    None,
}

impl Object {
    fn as_string(&self) -> Option<&String> {
        match self {
            Object::String(s) => Some(s),
            _ => None,
        }
    }

    fn as_map(&self) -> Option<&HashMap<String, Object>> {
        match self {
            Object::Map(m) => Some(m),
            _ => None,
        }
    }
}

// Implémentez les méthodes nécessaires pour ContractModel
impl ContractModel {
    fn parse(attributes: &HashMap<String, Object>) -> Self {
        // Implémentez la logique de parsing ici
        ContractModel { /* champs */ }
    }

    fn get_as(&self, attribute: &str) -> &str {
        // Implémentez la logique pour obtenir l'attribut ici
        ""
    }
}