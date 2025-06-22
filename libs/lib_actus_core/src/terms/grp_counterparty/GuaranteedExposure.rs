use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_counterparty::guaranteed_exposure::MV::MV;
use crate::terms::grp_counterparty::guaranteed_exposure::NI::NI;
use crate::terms::grp_counterparty::guaranteed_exposure::NO::NO;
use crate::exceptions::ParseError::ParseError;

#[derive(PartialEq, Eq, Debug)]
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
    pub fn new_NO() -> Self {
        Self::NO(NO::new())
    }
    pub fn new_NI() -> Self {
        Self::NI(NI::new())
    }
    pub fn new_MV() -> Self {
        Self::MV(MV::new())
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

impl FromStr for GuaranteedExposure {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NO" => Ok(Self::NO(NO::new())),
            "NI" => Ok(Self::NI(NI::new())),
            "MV" => Ok(Self::MV(MV::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}

impl Default for GuaranteedExposure {
    fn default() -> Self {
        GuaranteedExposure::None
    }
}

