use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::terms::grp_counterparty::guaranteed_exposure::MV::MV;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use crate::util::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GuaranteedExposure {
    NO(NO),
    NI(NI),
    MV(MV),
    None
}

impl GuaranteedExposure {
    
    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => GuaranteedExposure::from_str(n),
            None => Ok(GuaranteedExposure::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
}

impl FromStr for GuaranteedExposure {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NO" => Ok( Self::NO(NO::new()  )),
            "NI" => Ok( Self::NI(NI::new()  )),
            "MV" => Ok( Self::MV(MV::new()  )),
            _ => Err(ParseError { message: format!("Invalid GuaranteedExposure: {}", s)})
        }
    }
}

impl Default for GuaranteedExposure {
    fn default() -> Self {
        GuaranteedExposure::None
    }
}

impl fmt::Display for GuaranteedExposure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NO(NO) => write!(f, "GuaranteedExposure: {}", NO.to_string()),
            Self::NI(NI) => write!(f, "GuaranteedExposure: {}", NI.to_string()),
            Self::MV(MV) => write!(f, "GuaranteedExposure: {}", MV.to_string()),
            Self::None => write!(f, "GuaranteedExposure: No value was given")
        }
    }
}