use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::guaranteed_exposure::MV::MV;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::exceptions::ParseError::ParseError;
use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::util::Value::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GuaranteedExposure {
    NO(NO),
    NI(NI),
    MV(MV),
    None
}

impl GuaranteedExposure {
    pub fn description(&self) -> String {
        match self {
            Self::NO(NO) => NO.type_str(),
            Self::NI(NI) => NI.type_str(),
            Self::MV(MV) => MV.type_str(),
            Self::None => "".to_string()
        }
    }


    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => GuaranteedExposure::from_str(n),
            None => Ok(GuaranteedExposure::None),
        }
    }
    
    pub fn provide(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        crate::util::CommonUtils::CommonUtils::provide(string_map, key)
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

