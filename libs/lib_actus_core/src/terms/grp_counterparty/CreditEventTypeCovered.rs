use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::util::Value::Value;

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

    pub fn new(element: &str) -> Result<Self, ParseError> {
        CreditEventTypeCovered::from_str(element)
    }

    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
    }
    pub fn provide_vec(string_map: &HashMap<String, Value>, key: &str) -> Option<Vec<Self>> {
        match string_map.get(key) {
            None => None, // Clé absente : valeur par défaut dans un Some
            Some(s) => {

                let  a =  s.as_vec().unwrap();
                //let a2 = CreditEventTypeCovered::from_str(a.get(0)?.as_str()).unwrap();

                let b0: Vec<CreditEventTypeCovered> = a.iter().map(|s| {    CreditEventTypeCovered::from_str(s.to_string().as_str()).unwrap()   }).collect();
                let b: Vec<Result<CreditEventTypeCovered, ParseError>> = a.iter().map(|s| {    CreditEventTypeCovered::from_str(s.to_string().as_str())   }).collect();
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
        }

    }
}

impl FromStr for CreditEventTypeCovered {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DL" => Ok(Self::DL(DL::new())),
            "DQ" => Ok( Self::DQ(DQ::new())),
            "DF" => Ok(Self::DF(DF::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for CreditEventTypeCovered {
    fn default() -> Self {
        Self::DF(DF::new())
    }
}

