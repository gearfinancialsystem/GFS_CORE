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
use crate::terms::grp_counterparty::seniority::S::S;

#[derive(Debug, PartialEq)]
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


    pub fn new(contract_type: &String) -> Self {

        match contract_type.as_str() {
            "PAM" => ContractType::PAM(PAM::default()),
            _ => ContractType::None
        }

    }
    
}





impl Default for ContractType {
    fn default() -> Self {
        Self::None
    }
}

