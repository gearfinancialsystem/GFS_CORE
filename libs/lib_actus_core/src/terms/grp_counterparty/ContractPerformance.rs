use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::AttributeConversionException::AttributeConversionException;
use crate::exceptions::ParseError::ParseError;

use crate::terms::grp_counterparty::contract_performance::Pf::PF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Ma::MA;
use crate::terms::grp_counterparty::contract_performance::Te::TE;

use crate::util::CommonUtils::{CommonUtils as cu, Value};

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

    pub fn description(&self) -> String {
        match self {
            Self::PF(PF) => PF.type_str(),
            Self::DL(DL) => DL.type_str(),
            Self::DQ(DQ) => DQ.type_str(),
            Self::DF(DF) => DF.type_str(),
            Self::MA(MA) => MA.type_str(),
            Self::TE(TE) => TE.type_str()
        }
    }
    pub fn new_PF() -> Self {
        Self::PF(PF::new())
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
    pub fn new_MA() -> Self {
        Self::MA(MA::new())
    }
    pub fn new_TE() -> Self {
        Self::TE(TE::new())
    }

    pub fn provide_box(string_map: &HashMap<String, String>, key: &str) -> Option<Box<Self>> {
        // on stock dans Rc car business day convention cont_type va aussi l'utiliser et la modifier
        string_map
            .get(key)
            .and_then(|s| {
                Self::from_str(s).ok()
            })
            .map(|b| Box::new(b)) // On stocke la convention dans une Box
            //.unwrap_or_default()
    }
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        cu::provide(string_map, key)
    }
    pub fn to_stringx(&self) -> Result<String, ParseError> {
        match self {
            Self::PF(PF) => Ok("PF".to_string()),
            Self::DL(DL) => Ok("DL".to_string()),
            Self::DQ(DQ) => Ok("DQ".to_string()),
            Self::DF(DF) => Ok("DF".to_string()),
            Self::MA(MA) => Ok("MA".to_string()),
            Self::TE(TE) => Ok("TE".to_string()),
            _ => Err(ParseError { message: format!("Invalid TOSTRING ContractPerformance ")})
        }

    }
}

impl FromStr for ContractPerformance {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PF" => Ok(Self::new_PF()),
            "DL" => Ok(Self::new_DL()),
            "DQ" => Ok(Self::new_DQ()),
            "DF" => Ok(Self::new_DF()),
            "MA" => Ok(Self::new_MA()),
            "TE" => Ok(Self::new_TE()),
            _ => Err(ParseError { message: format!("Invalid ContractPerformance: {}", s)})
        }
    }
}

impl Default for ContractPerformance {
    fn default() -> Self {
        Self::new_PF()
    }
}

