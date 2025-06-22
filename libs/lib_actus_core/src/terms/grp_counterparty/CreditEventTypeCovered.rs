use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::contract_performance::Df::DF;
use crate::terms::grp_counterparty::contract_performance::Dl::DL;
use crate::terms::grp_counterparty::contract_performance::Dq::DQ;
use crate::exceptions::ParseError::ParseError;



#[derive(Debug, Eq, PartialEq)]
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

