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
use crate::terms::grp_fees::FeeBasis::FeeBasis;

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

    pub fn new(element: Option<&str>) -> Result<Self, ParseError> {
        match element {
            Some(n) => Unit::from_str(n),
            None => Ok(Unit::None),
        }
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
            "BRL" => Ok(Self::BRL(BRL::new())),
            "BSH" => Ok(Self::BSH(BSH::new())),
            "GLN" => Ok(Self::GLN(GLN::new())),
            "CUU" => Ok(Self::CUU(CUU::new())),
            "MWH" => Ok(Self::MWH(MWH::new())),
            "PND" => Ok(Self::PND(PND::new())),
            "STN" => Ok(Self::STN(STN::new())),
            "TON" => Ok(Self::TON(TON::new())),
            "TRO" => Ok(Self::TRO(TRO::new())),
            _ => Err(ParseError { message: format!("Invalid BusinessDayAdjuster: {}", s)})
        }
    }
}
