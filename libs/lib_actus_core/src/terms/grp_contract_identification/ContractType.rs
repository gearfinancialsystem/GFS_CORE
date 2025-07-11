
use crate::attributes::ContractModel::ContractModel;

use crate::terms::grp_contract_identification::contract_types::Pam::PAM;
use crate::terms::grp_contract_identification::contract_types::Swaps::SWAPS;
use crate::terms::grp_contract_identification::contract_types::Ann::ANN;
// use crate::terms::grp_contract_identification::contract_types::Bcs::BCS;
// use crate::terms::grp_contract_identification::contract_types::Cec::CEC;
// use crate::terms::grp_contract_identification::contract_types::Com::COM;
// use crate::terms::grp_contract_identification::contract_types::Capfl::CAPFL;
// use crate::terms::grp_contract_identification::contract_types::Ceg::CEG;
// use crate::terms::grp_contract_identification::contract_types::Clm::CLM;
// use crate::terms::grp_contract_identification::contract_types::Csh::CSH;
// use crate::terms::grp_contract_identification::contract_types::Futur::FUTUR;
// use crate::terms::grp_contract_identification::contract_types::Fxout::FXOUT;
use crate::terms::grp_contract_identification::contract_types::Lam::LAM;
// use crate::terms::grp_contract_identification::contract_types::Lax::LAX;
use crate::terms::grp_contract_identification::contract_types::Nam::NAM;
// use crate::terms::grp_contract_identification::contract_types::Optns::OPTNS;
// use crate::terms::grp_contract_identification::contract_types::Stk::STK;
// use crate::terms::grp_contract_identification::contract_types::Swppv::SWPPV;
// use crate::terms::grp_contract_identification::contract_types::Ump::UMP;

use crate::types::IsoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(PartialEq, Debug, Clone)]
pub struct ContractType;

impl ContractType {


    pub fn schedule(to: Option<IsoDatetime>, cm: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {

        match cm.contract_type.clone().as_str() {
            "PAM" => PAM::schedule(to, cm),
            "LAM" => LAM::schedule(to, cm),
            "NAM" => NAM::schedule(to, cm),
            "ANN" => ANN::schedule(to, cm),
            //"LAX" => Some(LAX::schedule(&to.unwrap(),cm).unwrap()),
            // "CLM" => Some(CLM::schedule(&to.unwrap(),cm).unwrap()),
            // "UMP" => Some(UMP::schedule(&to.unwrap(),cm).unwrap()),
            // "CSH" => Some(CSH::schedule(&to.unwrap(),cm).unwrap()),
            // "STK" => Some(STK::schedule(&to.unwrap(),cm).unwrap()),
            // "COM" => Some(COM::schedule(&to.unwrap(),cm).unwrap()),
            // "FXOUT" => Some(FXOUT::schedule(&to.unwrap(),cm).unwrap()),
            // "SWPPV" => Some(SWPPV::schedule(&to.unwrap(),cm).unwrap()),
            "SWAPS" => SWAPS::schedule(to,cm),
            // "CAPFL" => Some(CAPFL::schedule(&to.unwrap(),cm).unwrap()),
            // "OPTNS" => Some(OPTNS::schedule(&to.unwrap(),cm).unwrap()),
            // "FUTUR" => Some(FUTUR::schedule(&to.unwrap(),cm).unwrap()),
            // "CEG" => Some(CEG::schedule(&to.unwrap(),cm).unwrap()),
            // "CEC" => Some(CEC::schedule(&to.unwrap(),cm).unwrap()),
            // "BCS" => Some(BCS::schedule(&to.unwrap(),cm).unwrap()),
            _ => Err("invalid contract type".to_string())
        }

    }
    pub fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>, cm: &ContractModel, observer: &RiskFactorModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String> {

        match cm.contract_type.clone().as_str() {
            "ANN" => ANN::apply(events, cm, observer),
            // "BCS" => Some(BCS::apply(events, cm, observer)),
            // "CPFL" => Some(CAPFL::apply(events, cm, observer)),
            // "CEC" => Some(CEC::apply(events, cm, observer)),
            // "CEG" => Some(CEG::apply(events, cm, observer)),
            // "CLM" => Some(CLM::apply(events, cm, observer)),
            // "CSH" => Some(CSH::apply(events, cm, observer)),
            // "FUTUR" => Some(FUTUR::apply(events, cm, observer)),
            // "FXOUT" => Some(FXOUT::apply(events, cm, observer)),
            "LAM" => LAM::apply(events, cm, observer),
            //"LAX" => Some(LAX::apply(events, cm, observer)),
            "NAM" => NAM::apply(events, cm, observer),
            // "OPTNS" => Some(OPTNS::apply(events, cm, observer)),
            "PAM" => PAM::apply(events, cm, observer),
            // "STK" => Some(STK::apply(events, cm, observer)),
            "SWAPS" => SWAPS::apply(events, cm, observer),
            // "SWPPV" => Some(SWPPV::apply(events, cm, observer)),
            // "UMP" => Some(UMP::apply(events, cm, observer)),
            _ => Err("invalid contract type".to_string())
        }
    }
}


// impl fmt::Display for ContractType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::NC(nc) => write!(f, "{}", nc.to_string()),
//             Self::MF(mf) => write!(f, "{}", mf.to_string()),
//         }
//     }
// }

