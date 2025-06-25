use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_counterparty::contract_performance::Ma::MA;
use crate::terms::grp_counterparty::contract_performance::Pf::PF;
use crate::terms::grp_counterparty::contract_performance::Te::TE;
use crate::terms::grp_reset_rate::ArrayFixedVariable::ArrayFixedVariable;
use crate::util::CommonUtils::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreditEventTypeCovered {
    DL(DL),
    DQ(DQ),
    DF(DF)
}
impl CreditEventTypeCovered {
    pub fn description(&self) -> String {
        match self {
            Self::DL(DL) => DL.type_str(),
            Self::DQ(DQ) => DQ.type_str(),
            Self::DF(DF) => DF.type_str(),
        }
    }
    pub fn new_DL() -> Self {
        Self::DL(DL::new())
    }
    pub fn new_DQ() -> Self {
        Self::DQ(DQ::new())
    }
    pub fn new_DF() -> Self {
        Self::DF(DF::new())
    }
    
    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Box<Self> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            .unwrap_or_default()
    }
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
    pub fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<Self>> {
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {

                let  a =  s.extract_vec_str().unwrap();
                let a2 = CreditEventTypeCovered::from_str(a.get(0)?.as_str()).unwrap();

                let b0: Vec<CreditEventTypeCovered> = a.iter().map(|s| {    CreditEventTypeCovered::from_str(s.as_str()).unwrap()   }).collect();
                let b: Vec<Result<CreditEventTypeCovered, Err>> = a.iter().map(|s| {    CreditEventTypeCovered::from_str(s.as_str())   }).collect();
                let c = b.iter().any(|r| r.is_err());

                if c == true {
                    panic!("Erreur de parsing pour la clé  avec la valeur ")
                } else {
                    Some(b0)
                }

            }
        }
    }
    pub fn to_stringx(&self) -> Result<String, ParseError> {
        match self {
            Self::DL(DL) => Ok("DL".to_string()),
            Self::DQ(DQ) => Ok("DQ".to_string()),
            Self::DF(DF) => Ok("DF".to_string()),
            _ => Err(ParseError { message: format!("Invalid TOSTRING ContractPerformance ")})
        }

    }
}

impl FromStr for CreditEventTypeCovered {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DL" => Ok(Self::new_DL()),
            "DQ" => Ok(Self::new_DQ()),
            "DF" => Ok(Self::new_DF()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for CreditEventTypeCovered {
    fn default() -> Self {
        Self::new_DF()
    }
}

