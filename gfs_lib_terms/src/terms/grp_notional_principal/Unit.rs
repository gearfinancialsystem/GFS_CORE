use std::collections::HashMap;
use std::fmt;
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

use gfs_lib_types::types::Value::Value;

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

    pub fn new(element: Option<&str>) -> Result<Self, String> {
        match element {
            Some(n) => Unit::from_str(n),
            None => Ok(Unit::None),
        }
    }

    pub fn provide_from_input_dict(string_map: &HashMap<String, Value>, key: &str) -> Option<Self> {
        match string_map.get(key) {
            None => None,// A VERIFIER // Clé absente : valeur par défaut dans un Some
            Some(s) => {
                match Self::from_str(s.as_string().unwrap().as_str()) {
                    Ok(value) => Some(value), // Valeur valide
                    Err(_) => panic!("Erreur de parsing pour la clé {:?} avec la valeur {:?}", key, s),
                }
            }
        }
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for Unit {
    type Err = String;
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
            _ => Err(format!("Invalid BusinessDayAdjuster: {}", s))
        }
    }
}
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BRL(v) => write!(f, "Unit: {}", v.to_string()),
            Self::BSH(v) => write!(f, "Unit: {}", v.to_string()),
            Self::GLN(v) => write!(f, "Unit: {}", v.to_string()),
            Self::CUU(v) => write!(f, "Unit: {}", v.to_string()),
            Self::MWH(v) => write!(f, "Unit: {}", v.to_string()),
            Self::PND(v) => write!(f, "Unit: {}", v.to_string()),
            Self::STN(v) => write!(f, "Unit: {}", v.to_string()),
            Self::TON(v) => write!(f, "Unit: {}", v.to_string()),
            Self::TRO(v) => write!(f, "Unit: {}", v.to_string()),
            Self::None => write!(f, "Unit: None"),
        }
    }
}