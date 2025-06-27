use std::collections::HashMap;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_counterparty::contract_performance::Pf::PF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Ma::MA;
use crate::terms::grp_counterparty::contract_performance::Te::TE;

use crate::util::CommonUtils::CommonUtils as cu;
use crate::util::Value::Value;
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


    pub fn new(element: &str) -> Result<Self, ParseError> {
        ContractPerformance::from_str(element)
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
            "PF" => Ok(Self::PF(PF::new())),
            "DL" => Ok(Self::DL(DL::new())),
            "DQ" => Ok(Self::DQ(DQ::new())),
            "DF" => Ok(Self::DF(DF::new())),
            "MA" => Ok(Self::MA(MA::new())),
            "TE" => Ok(Self::TE(TE::new())),
            _ => Err(ParseError { message: format!("Invalid ContractPerformance: {}", s)})
        }
    }
}

impl Default for ContractPerformance {
    fn default() -> Self {
        Self::PF(PF::new())
    }
}

