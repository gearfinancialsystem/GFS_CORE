use std::collections::HashMap;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_types::types::Value::Value;
use crate::contracts::Ann::ANN;
use crate::contracts::Bcs::BCS;
use crate::contracts::Capfl::CAPFL;
use crate::contracts::Cec::CEC;
use crate::contracts::Ceg::CEG;
use crate::contracts::Clm::CLM;
use crate::contracts::Com::COM;
use crate::contracts::Csh::CSH;
use crate::contracts::Futur::FUTUR;
use crate::contracts::Pam::PAM;
use crate::contracts::Swaps::SWAPS;
use crate::contracts::Fxout::FXOUT;
use crate::contracts::Lam::LAM;
use crate::contracts::Lax::LAX;
use crate::contracts::Nam::NAM;
use crate::contracts::Optns::OPTNS;
use crate::contracts::Stk::STK;
use crate::contracts::Swppv::SWPPV;
use crate::contracts::Ump::UMP;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub enum ContractModel {
    ANN(ANN),
    BCS(BCS),
    CAPFL(CAPFL),
    CEC(CEC),
    CEG(CEG),
    CLM(CLM),
    COM(COM),
    CSH(CSH),
    FUTUR(FUTUR),
    FXOUT(FXOUT),
    LAM(LAM),
    LAX(LAX),
    NAM(NAM),
    OPTNS(OPTNS),
    PAM(PAM),
    STK(STK),
    SWAPS(SWAPS),
    SWPPV(SWPPV),
    UMP(UMP),

}

impl ContractModel {
    
    pub fn new(sm_terms: &HashMap<String, Value>,
               risk_factors: &Option<RiskFactorModel>) -> Result<ContractModel, String> {
        let ct = sm_terms.get("contractType").unwrap().as_string().unwrap().as_str();
        match ct {
            "ANN" => {
                Ok(Self::ANN({
                    let mut c = ANN::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "BCS" => {
                Ok(Self::BCS({
                    let mut c = BCS::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "CAPFL" => {
                Ok(Self::CAPFL({
                    let mut c = CAPFL::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "CEC" => {
                Ok(Self::CEC({
                    let mut c = CEC::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "CEG" => {
                Ok(Self::CEG({
                    let mut c = CEG::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "CLM" => {
                Ok(Self::CLM({
                    let mut c = CLM::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "COM" => {
                Ok(Self::COM({
                    let mut c = COM::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "CSH" => {
                Ok(Self::CSH({
                    let mut c = CSH::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "FUTUR" => {
                Ok(Self::FUTUR({
                    let mut c = FUTUR::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "FXOUT" => {
                Ok(Self::FXOUT({
                    let mut c = FXOUT::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                })) },
            "LAM" => {
                Ok(Self::LAM({
                    let mut c = LAM::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                })) },
            "LAX" => {
                Ok(Self::LAX({
                    let mut c = LAX::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                })) },
            "NAM" => {
                Ok(Self::NAM({
                    let mut c = NAM::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                })) },
            "OPTNS" => {
                Ok(Self::OPTNS({
                    let mut c = OPTNS::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                })) },
            "PAM" => {
                Ok(Self::PAM({
                    let mut c = PAM::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "STK" => {
                Ok(Self::STK({
                    let mut c = STK::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);
                    c
                }))
            },
            "SWAPS" => {
                Ok(Self::SWAPS({
                    let mut c = SWAPS::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                }))

            },
            "SWPPV" => {
                Ok(Self::SWPPV({
                    let mut c = SWPPV::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                }))

            },
            "UMP" => {
                Ok(Self::UMP({
                    let mut c = UMP::new();
                    c.set_contract_terms(sm_terms);
                    c.set_contract_risk_factors(risk_factors);
                    c.set_contract_structure(sm_terms);

                    c
                }))

            },


            _ => Err(format!("Unknown contract type {}", ct))
        }
    }
    
    pub fn run_schedule(&mut self, to: Option<IsoDatetime>) {
        match self {
            ContractModel::ANN(c) => {c.schedule(to)},
            ContractModel::BCS(c) => {c.schedule(to)},
            ContractModel::CAPFL(c) => {c.schedule(to)},
            ContractModel::CEC(c) => {c.schedule(to)},
            ContractModel::CEG(c) => {c.schedule(to)},
            ContractModel::CLM(c) => {c.schedule(to)},
            ContractModel::COM(c) => {c.schedule(to)},
            ContractModel::CSH(c) => {c.schedule(to)},
            ContractModel::FUTUR(c) => {c.schedule(to)},
            ContractModel::FXOUT(c) => {c.schedule(to)},
            ContractModel::LAM(c) => {c.schedule(to)},
            ContractModel::LAX(c) => {c.schedule(to)},
            ContractModel::NAM(c) => {c.schedule(to)},
            ContractModel::OPTNS(c) => {c.schedule(to)},
            ContractModel::PAM(c) => {c.schedule(to)},
            ContractModel::STK(c) => {c.schedule(to)},
            ContractModel::SWAPS(c) => {c.schedule(to)},
            ContractModel::SWPPV(c) => {c.schedule(to)},
            ContractModel::UMP(c) => {c.schedule(to)},
        }
    }

    pub fn run_apply(&mut self, result_set_toogle: bool) {
        match self {
            ContractModel::ANN(c) => {c.apply(result_set_toogle)},
            ContractModel::BCS(c) => {c.apply(result_set_toogle)},
            ContractModel::CAPFL(c) => {c.apply(result_set_toogle)},
            ContractModel::CEC(c) => {c.apply(result_set_toogle)},
            ContractModel::CEG(c) => {c.apply(result_set_toogle)},
            ContractModel::CLM(c) => {c.apply(result_set_toogle)},
            ContractModel::COM(c) => {c.apply(result_set_toogle)},
            ContractModel::CSH(c) => {c.apply(result_set_toogle)},
            ContractModel::FUTUR(c) => {c.apply(result_set_toogle)},
            ContractModel::FXOUT(c) => {c.apply(result_set_toogle)},
            ContractModel::LAM(c) => {c.apply(result_set_toogle)},
            ContractModel::LAX(c) => {c.apply(result_set_toogle)},
            ContractModel::NAM(c) => {c.apply(result_set_toogle)},
            ContractModel::OPTNS(c) => {c.apply(result_set_toogle)},
            ContractModel::PAM(c) => {c.apply(result_set_toogle)},
            ContractModel::STK(c) => {c.apply(result_set_toogle)},
            ContractModel::SWAPS(c) => {c.apply(result_set_toogle)},
            ContractModel::SWPPV(c) => {c.apply(result_set_toogle)},
            ContractModel::UMP(c) => {c.apply(result_set_toogle)},

        }
    }

    pub fn run(&mut self, to: Option<IsoDatetime>, result_set_toogle: bool) {
        self.run_schedule(to);
        self.run_apply(result_set_toogle);
    }

}