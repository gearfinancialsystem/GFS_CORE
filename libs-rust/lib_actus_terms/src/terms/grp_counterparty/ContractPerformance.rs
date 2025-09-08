use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::terms::grp_counterparty::contract_performance::Pf::PF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Ma::MA;
use crate::terms::grp_counterparty::contract_performance::Te::TE;



use lib_actus_types::types::Value::Value;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractPerformance {
    PF(PF),
    DL(DL),
    DQ(DQ),
    DF(DF),
    MA(MA),
    TE(TE),
}

impl ContractPerformance {
    
    pub fn new(element: &str) -> Result<Self, String> {
        ContractPerformance::from_str(element)
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        // cu::provide(string_map, key)
        crate::utils::ProvideFuncs::provide(string_map, key)
    }
    pub fn to_stringx(&self) -> Result<String, String> {
        match self {
            Self::PF(PF) => Ok("PF".to_string()),
            Self::DL(DL) => Ok("DL".to_string()),
            Self::DQ(DQ) => Ok("DQ".to_string()),
            Self::DF(DF) => Ok("DF".to_string()),
            Self::MA(MA) => Ok("MA".to_string()),
            Self::TE(TE) => Ok("TE".to_string()),
            _ => Err(format!("Invalid TOSTRING ContractPerformance "))
        }

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

impl FromStr for ContractPerformance {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PF" => Ok(Self::PF(PF::new())),
            "DL" => Ok(Self::DL(DL::new())),
            "DQ" => Ok(Self::DQ(DQ::new())),
            "DF" => Ok(Self::DF(DF::new())),
            "MA" => Ok(Self::MA(MA::new())),
            "TE" => Ok(Self::TE(TE::new())),
            _ => Err(format!("Invalid ContractPerformance: {}", s))
        }
    }
}

impl fmt::Display for ContractPerformance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PF(PF) => write!(f, "Contract Performance: {}", PF.to_string()),
            Self::DL(DL) => write!(f, "Contract Performance: {}", DL.to_string()),
            Self::DQ(DQ) => write!(f, "Contract Performance: {}", DQ.to_string()),
            Self::DF(DF) => write!(f, "Contract Performance: {}", DF.to_string()),
            Self::MA(MA) => write!(f, "Contract Performance: {}", MA.to_string()),
            Self::TE(TE) => write!(f, "Contract Performance: {}", TE.to_string()),

        }
    }
}
impl Default for ContractPerformance {
    fn default() -> Self {
        Self::PF(PF::new())
    }
}

