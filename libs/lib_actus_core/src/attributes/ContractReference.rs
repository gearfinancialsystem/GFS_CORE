use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use chrono::Days;
use crate::attributes::reference_role::ReferenceRole::ReferenceRole;
use crate::attributes::reference_type::ReferenceType::ReferenceType;
use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::functions::pam::pof::POF_AD_PAM::POF_AD_PAM;
use crate::functions::pam::stf::STF_AD_PAM::STF_AD_PAM;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util::Value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    ContractModel(ContractModel),
    Map(HashMap<String, Object>),
    None,
}

impl Object {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_cm(&self) -> Option<ContractModel> {
        match self {
            Self::ContractModel(m) => Some(m.clone()),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Object>> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }
}

// Structure pour représenter ContractReference
#[derive(Clone, Debug, PartialEq)]
pub struct ContractReference {
    pub reference_role: ReferenceRole,
    pub reference_type: ReferenceType,
    pub object: Object,
}

impl ContractReference {
    pub fn new(attributes: &HashMap<String, Value>, contract_role: &ContractRole) -> Self {
        let reference_role = ReferenceRole::from_str(attributes.get("referenceRole").unwrap().as_string().unwrap().as_str()).unwrap();
        let reference_type = ReferenceType::from_str(attributes.get("referenceType").unwrap().as_string().unwrap().as_str()).unwrap();
        let object = match reference_type {
            ReferenceType::CNT => {
                let mut child_model = attributes.get("object").unwrap().to_hashmap();
                match (contract_role, &reference_role) {
                    (ContractRole::RFL(RFL), ReferenceRole::FIL) => {
                        child_model.insert("contractRole".to_string(), Value::Vstring("RPA".to_string()));
                    },
                    (ContractRole::RFL(RFL), _) => {
                        child_model.insert("contractRole".to_string(), Value::Vstring("RPL".to_string()));
                    },
                    (_, ReferenceRole::FIL) => {
                        child_model.insert("contractRole".to_string(), Value::Vstring("RPL".to_string()));
                    },
                    (_, _) => {
                        child_model.insert("contractRole".to_string(), Value::Vstring("RPA".to_string()));
                    }
                }
                Object::ContractModel(ContractModel::new(&child_model).unwrap())
            },
            ReferenceType::CID => {
                Object::String(attributes.get("object").unwrap().as_hashmap().unwrap().get("contract_identifier").unwrap().to_string())
            },
            ReferenceType::MOC => {
                Object::String(attributes.get("object").unwrap().as_hashmap().unwrap().get("marketObjectCode").unwrap().to_string())
            },
            ReferenceType::EID => {
                Object::None // a implementer //attributes.get("object").unwrap().clone()
            },
            ReferenceType::CST => {
                Object::None
            }
        };

        ContractReference { reference_role, reference_type, object }
    }

    pub fn get_object(&self) -> &Object {
        &self.object
    }

    // pub fn get_contract_attribute(&self, contract_attribute: &str) -> Option<String> {
    //     match &self.object {
    //         Object::String(s) if contract_attribute == "marketObjectCode" && matches!(self.reference_type, ReferenceType::MOC) => Some(s.clone()),
    //         Object::ContractModel(model) if matches!(self.reference_type, ReferenceType::CNT) => Some(model.get_field(contract_attribute)?.extract_vString()?),
    //
    //         Object::String(s) if matches!(self.reference_type, ReferenceType::CID) => Some(s.clone()),
    //         _ => None,
    //     }
    // }

    pub fn get_state_space_at_time_point(&self, time: IsoDatetime, observer: &RiskFactorModel) -> StateSpace {
        let model = self.object.as_cm().unwrap();
        if self.reference_type == ReferenceType::CID {
            let mut events =  ContractType::schedule( Some(time.checked_add_days(Days::new(1))).unwrap(), &self.object.as_cm().unwrap()).unwrap();
            let analysis_event = EventFactory::create_event(
                &Some(time),
                &EventType::AD,
                &model.currency,
                Some(Rc::new(POF_AD_PAM)),
                Some(Rc::new(STF_AD_PAM)),
                &None,
                &model.contract_id
            );
            events.push(analysis_event.clone());
            events.sort();
            events = ContractType::apply(events, &self.object.as_cm().unwrap(), observer).unwrap();
            analysis_event.states();
        }
        StateSpace::default() // a checker
    }

    pub fn get_event(&self, event_type: EventType, time: IsoDatetime, observer: &RiskFactorModel ) -> ContractEvent<IsoDatetime, IsoDatetime> {
        let mut events : Vec<ContractEvent<IsoDatetime, IsoDatetime>> = vec![];
        if self.reference_type == ReferenceType::CNT {
            events = ContractType::apply(
                ContractType::schedule(None, &self.object.as_cm().unwrap()).unwrap(),
                &self.object.as_cm().unwrap(),
                observer
            ).unwrap();
            events.iter_mut().filter(|e| e.event_type == event_type);
        }
        events.get(0).unwrap().clone()
    }

    // Ajoutez d'autres méthodes si nécessaire
}
