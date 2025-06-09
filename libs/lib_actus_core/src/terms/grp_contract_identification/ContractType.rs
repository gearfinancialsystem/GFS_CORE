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
use crate::util::ParseError::ParseError;


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

impl TraitTermDescription for ContractType {
    fn get_identifier(&self) -> &str {
        "contractType"
    }
    fn get_group(&self) -> &str {
        "Contract identification"
    }
    fn get_name(&self) -> &str {
        "Contract Type"
    }
    fn get_acronym(&self) -> &str {
        "CT"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'principalAtMaturity', 'name': 'Principal at Maturity', 'acronym': 'PAM', 'description': 'Lending agreements with full amortization at maturity.\r'}, {'option': '1', 'identifier': 'annuity', 'name': 'Annuity', 'acronym': 'ANN', 'description': 'Lending agreements with fixed periodic payments consisting of an interest and principal portion. The periodic payments are adjusted for variable rate instruments such that maturity remains fixed.\r'}, {'option': '2', 'identifier': 'negativeAmortizer', 'name': 'Negative Amortizer', 'acronym': 'NAM', 'description': 'Lending agreements with fixed periodic payments consisting of an interest and principal portion. Maturity changes for variable rate instruments. \r'}, {'option': '3', 'identifier': 'linearAmortizer', 'name': 'Linear Amortizer', 'acronym': 'LAM', 'description': 'Lending agreements with fixed principal repayment amounts and variable interest payments.\r'}, {'option': '4', 'identifier': 'exoticLinearAmortizer', 'name': 'Exotic Linear Amortizer', 'acronym': 'LAX', 'description': 'Lending agreements with exotic repayment schedules.\r'}, {'option': '5', 'identifier': 'callMoney', 'name': 'Call Money', 'acronym': 'CLM', 'description': 'Lonas that are rolled over as long as they are not called. Once called it has to be paid back after the stipulated notice period.\r'}, {'option': '6', 'identifier': 'undefinedMaturityProfile', 'name': 'Undefined Maturity Profile', 'acronym': 'UMP', 'description': 'Interest paying cash accounts (current / savings / etc.). \r'}, {'option': '7', 'identifier': 'cash', 'name': 'Cash', 'acronym': 'CSH', 'description': 'Represents cash holdings. \r'}, {'option': '8', 'identifier': 'stock', 'name': 'Stock', 'acronym': 'STK', 'description': 'Represents stocks/shares/equity. \r'}, {'option': '9', 'identifier': 'commodity', 'name': 'Commodity', 'acronym': 'COM', 'description': 'Represents commodities. \r'}, {'option': '10', 'identifier': 'swap', 'name': 'Swap', 'acronym': 'SWAPS', 'description': 'An agreement of swapping two legs such as fixed against variable or currency 1 against currency 2 etc. \r'}, {'option': '11', 'identifier': 'plainVanillaSwap', 'name': 'Plain Vanilla Swap', 'acronym': 'SWPPV', 'description': 'Plain vanilla interest rate swaps. \r'}, {'option': '12', 'identifier': 'foreignExchangeOutright', 'name': 'Foreign Exchange Outright', 'acronym': 'FXOUT', 'description': 'An agreement of swapping two cash flows in different currencies at a future point in time. \r'}, {'option': '13', 'identifier': 'capFloor', 'name': 'Cap and Floor', 'acronym': 'CAPFL', 'description': 'An agreement of paying the differential (cap or floor) of a reference rate versus a fixed rate. \r'}, {'option': '14', 'identifier': 'future', 'name': 'Future', 'acronym': 'FUTUR', 'description': 'An agreement of exchanging an underlying instrument against a fixed price in the future. \r'}, {'option': '15', 'identifier': 'option', 'name': 'Option', 'acronym': 'OPTNS', 'description': 'Different terms of options on buying an underlying instrument at a fixed price in the future. \r'}, {'option': '16', 'identifier': 'creditEnhancementGuarantee', 'name': 'Credit Enhancement Guarantee', 'acronym': 'CEG', 'description': 'A guarantee / letter of credit by a third party on the scheduled payment obligations of an underlying instrument \r'}, {'option': '17', 'identifier': 'creditEnhancementCollateral', 'name': 'Credit Enhancement Collateral', 'acronym': 'CEC', 'description': 'A collateral securing the scheduled payment obligations of an underlying instrument'}]"
    }
    fn get_default_value(&self) -> &str {
        ""
    }
    fn get_description(&self) -> &str {
        "The ContractType is the most important information. It defines the cash flow generating pattern of a contract. The ContractType information in combination with a given state of the risk factors will produce a deterministic sequence of cash flows which are the basis of any financial analysis."
    }
}    