use std::collections::HashMap;
use std::str::FromStr;
use crate::terms::grp_notional_principal::unitx::BRL::BRL;
use crate::terms::grp_notional_principal::unitx::BSH::BSH;
use crate::terms::grp_notional_principal::unitx::CUU::CUU;
use crate::terms::grp_notional_principal::unitx::GLN::GLN;
use crate::terms::grp_notional_principal::unitx::MWH::MWH;
use crate::terms::grp_notional_principal::unitx::PND::PND;
use crate::terms::grp_notional_principal::unitx::STN::STN;
use crate::terms::grp_notional_principal::unitx::TON::TON;
use crate::terms::grp_notional_principal::unitx::TRO::TRO;
use crate::exceptions::ParseError::ParseError;

pub enum Unit {
    BRL(BRL),
    BSH(BSH),
    GLN(GLN),
    CUU(CUU),
    MWH(MWH),
    PND(PND),
    STN(STN),
    TON(TON),
    TRO(TRO),
    None
}

impl Unit {
    pub fn description(&self) -> String {
        match self {
            Self::BRL(BRL) => BRL.type_str(),
            Self::BSH(BSH) => BSH.type_str(),
            Self::GLN(GLN) => GLN.type_str(),
            Self::CUU(CUU) => CUU.type_str(),
            Self::MWH(MWH) => MWH.type_str(),
            Self::PND(PND) => PND.type_str(),
            Self::STN(STN) => STN.type_str(),
            Self::TON(TON) => TON.type_str(),
            Self::TRO(TRO) => TRO.type_str(),
            Self::None => "".to_string(),
        }
    }
    pub fn new_BRL() -> Self {
        Self::BRL(BRL::new())
    }
    pub fn new_BSH() -> Self {
        Self::BSH(BSH::new())
    }
    pub fn new_GLN() -> Self {
        Self::GLN(GLN::new())
    }
    pub fn new_CUU() -> Self {
        Self::CUU(CUU::new())
    }
    pub fn new_MWH() -> Self {
        Self::MWH(MWH::new())
    }
    pub fn new_PND() -> Self {
        Self::PND(PND::new())
    }
    pub fn new_STN() -> Self {
        Self::STN(STN::new())
    }
    pub fn new_TON() -> Self {
        Self::TON(TON::new())
    }
    pub fn new_TRO() -> Self {
        Self::TRO(TRO::new())
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

impl Default for Unit {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for Unit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BRL" => Ok(Unit::new_BRL()),
            "BSH" => Ok(Unit::new_BSH()),
            "GLN" => Ok(Unit::new_GLN()),
            "CUU" => Ok(Unit::new_CUU()),
            "MWH" => Ok(Unit::new_MWH()),
            "PND" => Ok(Unit::new_PND()),
            "STN" => Ok(Unit::new_STN()),
            "TON" => Ok(Unit::new_TON()),
            "TRO" => Ok(Unit::new_TRO()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}
