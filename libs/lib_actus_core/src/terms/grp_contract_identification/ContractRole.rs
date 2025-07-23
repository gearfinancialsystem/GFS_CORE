use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::terms::grp_contract_identification::contract_roles::{
    Rpa::RPA, Rpl::RPL, Rfl::RFL, Pfl::PFL,
    Rf::RF, Pf::PF, Buy::BUY, Sel::SEL,
    Col::COL, Cno::CNO, Udl::UDL, Udlp::UDLP,
    Udlm::UDLM,
};
use crate::types::Value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractRole {
    RPA(RPA), RPL(RPL), RFL(RFL), PFL(PFL),
    RF(RF), PF(PF), BUY(BUY), SEL(SEL),
    COL(COL), CNO(CNO), UDL(UDL), UDLP(UDLP),
    UDLM(UDLM), None
}
impl ContractRole {

    pub fn role_sign(&self) -> f64 {
        match self {
            Self::RPA(RPA) => RPA.role_sign(),
            Self::RPL(RPL) => RPL.role_sign(),
            Self::RFL(RFL) => RFL.role_sign(),
            Self::PFL(PFL) => PFL.role_sign(),
            Self::RF(RF) => RF.role_sign(),
            Self::PF(PF) => PF.role_sign(),
            Self::BUY(BUY) => BUY.role_sign(),
            Self::SEL(SEL) => SEL.role_sign(),
            Self::COL(COL) => COL.role_sign(),
            Self::CNO(CNO) => CNO.role_sign(),
            Self::UDL(UDL) => UDL.role_sign(),
            Self::UDLP(UDLP) => UDLP.role_sign(),
            Self::UDLM(UDLM) => UDLM.role_sign(),
            Self::None => 0.0,
        }
    }

    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => ContractRole::from_str(n),
            None => Ok(ContractRole::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s.as_string().unwrap().as_str()).ok()
            })
            .map(|b| b) // On stocke la convention dans une Box
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

impl FromStr for ContractRole {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "RPA"  => Ok(Self::RPA(RPA::new())),
            "RPL"  => Ok(Self::RPL(RPL::new())),
            "RFL"  => Ok(Self::RFL(RFL::new())),
            "PFL"  => Ok(Self::PFL(PFL::new())),
            "RF"   => Ok(Self::RF(RF::new())),
            "PF"   => Ok(Self::PF(PF::new())),
            "BUY"  => Ok(Self::BUY(BUY::new())),
            "SEL"  => Ok(Self::SEL(SEL::new())),
            "COL"  => Ok(Self::COL(COL::new())),
            "CNO"  => Ok(Self::CNO(CNO::new())),
            "UDL"  => Ok(Self::UDL(UDL::new())),
            "UDLP" => Ok(Self::UDLP(UDLP::new())),
            "UDLM" => Ok(Self::UDLM(UDLM::new())),
            _ => Err(format!("Invalid BusinessDayAdjuster: {}", s))
        }
    }
}

impl fmt::Display for ContractRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RPA(RPA) => write!(f, "{}", RPA.to_string()),
            Self::RPL(RPL) => write!(f, "{}", RPL.to_string()),
            Self::RFL(RFL) => write!(f, "{}", RFL.to_string()),
            Self::PFL(PFL) => write!(f, "{}", PFL.to_string()),
            Self::RF(RF) => write!(f, "{}", RF.to_string()),
            Self::PF(PF) => write!(f, "{}", PF.to_string()),
            Self::BUY(BUY) => write!(f, "{}", BUY.to_string()),
            Self::SEL(SEL) => write!(f, "{}", SEL.to_string()),
            Self::COL(COL) => write!(f, "{}", COL.to_string()),
            Self::CNO(CNO) => write!(f, "{}", CNO.to_string()),
            Self::UDL(UDL) => write!(f, "{}", UDL.to_string()),
            Self::UDLP(UDLP) => write!(f, "{}", UDLP.to_string()),
            Self::UDLM(UDLM) => write!(f, "{}", UDLM.to_string()),
            Self::None => write!(f, "None")
        }
    }
}

impl Default for ContractRole {
    fn default() -> Self {
        Self::None
    }
}

