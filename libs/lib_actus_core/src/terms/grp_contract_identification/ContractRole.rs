use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_contract_identification::contract_roles::Rpa::RPA;
use crate::terms::grp_contract_identification::contract_roles::Rpl::RPL;
use crate::terms::grp_contract_identification::contract_roles::Rfl::RFL;
use crate::terms::grp_contract_identification::contract_roles::Pfl::PFL;
use crate::terms::grp_contract_identification::contract_roles::Rf::RF;
use crate::terms::grp_contract_identification::contract_roles::Pf::PF;
use crate::terms::grp_contract_identification::contract_roles::Buy::BUY;
use crate::terms::grp_contract_identification::contract_roles::Sel::SEL;
use crate::terms::grp_contract_identification::contract_roles::Col::COL;
use crate::terms::grp_contract_identification::contract_roles::Cno::CNO;
use crate::terms::grp_contract_identification::contract_roles::Udl::UDL;
use crate::terms::grp_contract_identification::contract_roles::Udlp::UDLP;
use crate::terms::grp_contract_identification::contract_roles::Udlm::UDLM;
use crate::util::Value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractRole {
    RPA(RPA),
    RPL(RPL),
    RFL(RFL),
    PFL(PFL),
    RF(RF),
    PF(PF),
    BUY(BUY),
    SEL(SEL),
    COL(COL),
    CNO(CNO),
    UDL(UDL),
    UDLP(UDLP),
    UDLM(UDLM),
    None
}
impl ContractRole {
    pub fn description(&self) -> String {
        match self {
            Self::RPA(RPA) => RPA.type_str(),
            Self::RPL(RPL) => RPL.type_str(),
            Self::RFL(RFL) => RFL.type_str(),
            Self::PFL(PFL) => PFL.type_str(),
            Self::RF(RF) => RF.type_str(),
            Self::PF(PF) => PF.type_str(),
            Self::BUY(BUY) => BUY.type_str(),
            Self::SEL(SEL) => SEL.type_str(),
            Self::COL(COL) => COL.type_str(),
            Self::CNO(CNO) => CNO.type_str(),
            Self::UDL(UDL) => UDL.type_str(),
            Self::UDLP(UDLP) => UDLP.type_str(),
            Self::UDLM(UDLM) => UDLM.type_str(),
            Self::None => "None".to_string(),
        }
    }

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
    

    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => ContractRole::from_str(n),
            None => Ok(ContractRole::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s.as_string().unwrap().as_str()).ok()
            })
            .map(|b| b) // On stocke la convention dans une Box
        //.unwrap_or_default()
    }
}

impl FromStr for ContractRole {
    type Err = ParseError;
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
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for ContractRole {
    fn default() -> Self {
        Self::None
    }
}

