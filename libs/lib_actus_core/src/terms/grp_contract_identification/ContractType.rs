use std::collections::HashMap;
use std::str::FromStr;

use crate::terms::grp_contract_identification::contract_types::Pam::PAM;
use crate::terms::grp_contract_identification::contract_types::Ann::ANN;
use crate::terms::grp_contract_identification::contract_types::Nam::NAM;
use crate::terms::grp_contract_identification::contract_types::Lam::LAM;
use crate::terms::grp_contract_identification::contract_types::Lax::LAX;
use crate::terms::grp_contract_identification::contract_types::Clm::CLM;
use crate::terms::grp_contract_identification::contract_types::Ump::UMP;
use crate::terms::grp_contract_identification::contract_types::Csh::CSH;
use crate::terms::grp_contract_identification::contract_types::Stk::STK;
use crate::terms::grp_contract_identification::contract_types::Com::COM;
use crate::terms::grp_contract_identification::contract_types::Swaps::SWAPS;
use crate::terms::grp_contract_identification::contract_types::Swppv::SWPPV;
use crate::terms::grp_contract_identification::contract_types::Fxout::FXOUT;
use crate::terms::grp_contract_identification::contract_types::Capfl::CAPFL;
use crate::terms::grp_contract_identification::contract_types::Futur::FUTUR;
use crate::terms::grp_contract_identification::contract_types::Optns::OPTNS;
use crate::terms::grp_contract_identification::contract_types::Ceg::CEG;
use crate::terms::grp_contract_identification::contract_types::Cec::CEC;
use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
use crate::traits::TraitTermDescription::TraitTermDescription;
use crate::exceptions::ParseError::ParseError;


#[derive(Debug, Eq, PartialEq)]
pub enum ContractType {
    PAM(PAM),
    //ANN(ANN),
    //NAM(NAM),
    //LAM(LAM),
    //LAX(LAX),
    // CLM(CLM),
    // UMP(UMP),
    // CSH(CSH),
    // STK(STK),
    // COM(COM),
    // SWAPS(SWAPS),
    // SWPPV(SWPPV),
    // FXOUT(FXOUT),
    // CAPFL(CAPFL),
    // FUTUR(FUTUR),
    // OPTNS(OPTNS),
    // CEG(CEG),
    // CEC(CEC),
    // BCS(BCS),
    None
}

impl ContractType {
    /// Décrit l'état actuel de l'enum en appelant `presentation` si nécessaire
    pub fn description(&self) -> String {
        match self {
            Self::PAM(PAM) => PAM.type_str(),
            // ContractType::ANN(ANN) => ANN.type_str(),
            // ContractType::NAM(NAM) => NAM.type_str(),
            // ContractType::LAM(LAM) => LAM.type_str(),
            // ContractType::LAX(LAX) => LAX.type_str(),
            // ContractType::CLM(CLM) => CLM.type_str(),
            // ContractType::UMP(UMP) => UMP.type_str(),
            // ContractType::CSH(CSH) => CSH.type_str(),
            // ContractType::STK(STK) => STK.type_str(),
            // ContractType::COM(COM)=> COM.type_str(),
            // ContractType::SWAPS(SWAPS) => SWAPS.type_str(),
            // ContractType::SWPPV(SWPPV) => SWPPV.type_str(),
            // ContractType::FXOUT(FXOUT) => FXOUT.type_str(),
            // ContractType::CAPFL(CAPFL) => CAPFL.type_str(),
            // ContractType::FUTUR(FUTUR) => FUTUR.type_str(),
            // ContractType::OPTNS(OPTNS) => OPTNS.type_str(),
            // ContractType::CEG(CEG) => CEG.type_str(),
            // ContractType::CEC(CEC) => CEC.type_str(),
            // ContractType::BCS(BCS) => BCS.type_str(),
            Self::None => "test".to_string()
        }
    }

    pub fn new_PAM() -> Self {
        Self::PAM(PAM::new())
    }

    // pub fn new_ANN() -> Self {
    //     ContractType::ANN(ANN::new())
    // }
    // 
    // pub fn new_NAM() -> Self {
    //     ContractType::NAM(NAM::new())
    // }
    // 
    // pub fn new_LAM() -> Self {
    //     ContractType::LAM(LAM::new())
    // }
    // 
    // pub fn new_LAX() -> Self {
    //     ContractType::LAX(LAX::new())
    // }
    // 
    // pub fn new_CLM() -> Self {
    //     ContractType::CLM(CLM::new())
    // }
    // 
    // pub fn new_UMP() -> Self {
    //     ContractType::UMP(UMP::new())
    // }
    // 
    // pub fn new_CSH() -> Self {
    //     ContractType::CSH(CSH::new())
    // }
    // 
    // pub fn new_STK() -> Self {
    //     ContractType::STK(STK::new())
    // }
    // 
    // pub fn new_COM() -> Self {
    //     ContractType::COM(COM::new())
    // }
    // 
    // pub fn new_SWAPS() -> Self {
    //     ContractType::SWAPS(SWAPS::new())
    // }
    // pub fn new_SWPPV() -> Self {
    //     ContractType::SWPPV(SWPPV::new())
    // }
    // 
    // pub fn new_FXOUT() -> Self {
    //     ContractType::FXOUT(FXOUT::new())
    // }
    // 
    // pub fn new_CAPFL() -> Self {
    //     ContractType::CAPFL(CAPFL::new())
    // }
    // 
    // pub fn new_FUTUR() -> Self {
    //     ContractType::FUTUR(FUTUR::new())
    // }
    // 
    // pub fn new_OPTNS() -> Self {
    //     ContractType::OPTNS(OPTNS::new())
    // }
    // 
    // pub fn new_CEG() -> Self {
    //     ContractType::CEG(CEG::new())
    // }
    // 
    // pub fn new_CEC() -> Self {
    //     ContractType::CEC(CEC::new())
    // }
    // 
    // pub fn new_BCS() -> Self {
    //     ContractType::BCS(BCS::new())
    // }

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
}



impl FromStr for ContractType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PAM" => Ok(Self::new_PAM()),
            // "ANN" => Ok(ContractType::new_ANN()),
            // "NAM" => Ok(ContractType::new_NAM()),
            // "LAM" => Ok(ContractType::new_LAM()),
            // "LAX" => Ok(ContractType::new_LAX()),
            // "CLM" => Ok(ContractType::new_CLM()),
            // "UMP" => Ok(ContractType::new_UMP()),
            // "CSH" => Ok(ContractType::new_CSH()),
            // "STK" => Ok(ContractType::new_STK()),
            // "COM" => Ok(ContractType::new_COM()),
            // "SWAPS" => Ok(ContractType::new_SWAPS()),
            // "SWPPV" => Ok(ContractType::new_SWPPV()),
            // "FXOUT" => Ok(ContractType::new_FXOUT()),
            // "CAPFL" => Ok(ContractType::new_CAPFL()),
            // "FUTUR" => Ok(ContractType::new_FUTUR()),
            // "OPTNS" => Ok(ContractType::new_OPTNS()),
            // "CEG" => Ok(ContractType::new_CEG()),
            // "CEC" => Ok(ContractType::new_CEC()),
            // "BCS" => Ok(ContractType::new_BCS()),
            _ => Err(ParseError {
                message: format!("Invalid Calendar cont_type: {}", s),
            }),
        }
    }
}


impl Default for ContractType {
    fn default() -> Self {
        Self::None
    }
}

