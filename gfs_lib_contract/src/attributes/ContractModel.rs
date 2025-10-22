use std::collections::HashMap;
use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use gfs_lib_types::types::Value::Value;
use crate::contracts::Pam::PAM;
use crate::contracts::Ann::ANN;
use crate::contracts::Fxout::FXOUT;
use crate::contracts::Lam::LAM;
use crate::contracts::Nam::NAM;
use crate::contracts::Swppv::SWPPV;
use crate::contracts::Stk::STK;
use crate::events::ContractEvent::ContractEvent;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContractModel {
    PAM(PAM),
    ANN(ANN),
    FXOUT(FXOUT),
    LAM(LAM),
    NAM(NAM),
    STK(STK),
    SWPPV(SWPPV),
    // BCS(BCS),
    // CAPFL(CAPFL),
    // CEC(CEC),
    // CEG(CEG),
    // CLM(CLM),
    // COM(COM),
    // CSH(CSH),
    // FUTUR(FUTUR),
    // LAX(LAX),
    // OPTNS(OPTNS),
    // SWAPS(SWAPS),
    // UMP(UMP),
}

impl ContractModel {
    
    pub fn new(sm_terms: HashMap<String, Value>,
               risk_factor_external_data : Option<Arc<dyn TraitExternalData>>,
               risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>,
               ) -> Result<ContractModel, String> { // result_set_toogle: bool
        let ct = sm_terms.get("contractType").unwrap().as_string().unwrap().as_str();
        match ct {
            "ANN" => {
                Ok(Self::ANN({
                    let mut c = ANN::new();
                    c.init_contract_terms(sm_terms.clone());
                    //c.set_result_vec(result_set_toogle);
                    c.init_risk_factor_external_data(risk_factor_external_data);
                    c.init_risk_factor_external_event(risk_factor_external_event);
                    c.init_related_contracts(sm_terms.clone());
                    c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
                    c.init_status_date();
                    c
                }))
            },
            // "BCS" => {
            //     Ok(Self::BCS({
            //         let mut c = BCS::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            // "CAPFL" => {
            //     Ok(Self::CAPFL({
            //         let mut c = CAPFL::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            // "CEC" => {
            //     Ok(Self::CEC({
            //         let mut c = CEC::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            // "CEG" => {
            //     Ok(Self::CEG({
            //         let mut c = CEG::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            // "CLM" => {
            //     Ok(Self::CLM({
            //         let mut c = CLM::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            // "COM" => {
            //     Ok(Self::COM({
            //         let mut c = COM::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            // "CSH" => {
            //     Ok(Self::CSH({
            //         let mut c = CSH::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            // "FUTUR" => {
            //     Ok(Self::FUTUR({
            //         let mut c = FUTUR::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            //         c
            //     }))
            // },
            "FXOUT" => {
                Ok(Self::FXOUT({
                    let mut c = FXOUT::new();
                    c.init_contract_terms(sm_terms.clone());
                    //c.set_result_vec(result_set_toogle);
                    c.init_risk_factor_external_data(risk_factor_external_data);
                    c.init_risk_factor_external_event(risk_factor_external_event);
                    c.init_related_contracts(sm_terms.clone());
                    c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
                    c.init_status_date();
                    c
                })) },
            "LAM" => {
                Ok(Self::LAM({
                    let mut c = LAM::new();
                    c.init_contract_terms(sm_terms.clone());
                    //c.set_result_vec(result_set_toogle);
                    c.init_risk_factor_external_data(risk_factor_external_data);
                    c.init_risk_factor_external_event(risk_factor_external_event);
                    c.init_related_contracts(sm_terms.clone());
                    c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
                    c.init_status_date();
                    c
                })) },
            // "LAX" => {
            //     Ok(Self::LAX({
            //         let mut c = LAX::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            // 
            //         c
            //     })) },
            "NAM" => {
                Ok(Self::NAM({
                    let mut c = NAM::new();
                    c.init_contract_terms(sm_terms.clone());
                    //c.set_result_vec(result_set_toogle);
                    c.init_risk_factor_external_data(risk_factor_external_data);
                    c.init_risk_factor_external_event(risk_factor_external_event);
                    c.init_related_contracts(sm_terms.clone());
                    c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
                    c.init_status_date();
                    c
                })) },
            // "OPTNS" => {
            //     Ok(Self::OPTNS({
            //         let mut c = OPTNS::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            // 
            //         c
            //     })) },
            "PAM" => {
                Ok(Self::PAM({
                    let mut c = PAM::new();
                    c.init_contract_terms(sm_terms.clone());
                    //c.set_result_vec(result_set_toogle);
                    c.init_risk_factor_external_data(risk_factor_external_data);
                    c.init_risk_factor_external_event(risk_factor_external_event);
                    c.init_related_contracts(sm_terms.clone());
                    c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
                    c.init_status_date();
                    c
                }))
            },
            "STK" => {
                Ok(Self::STK({
                    let mut c = STK::new();
                    c.init_contract_terms(sm_terms.clone());
                    c.init_risk_factor_external_data(risk_factor_external_data);
                    c.init_risk_factor_external_event(risk_factor_external_event);
                    c.init_related_contracts(sm_terms.clone());
                    c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
                    c.init_status_date();
                    c
                }))
            },
            // "SWAPS" => {
            //     Ok(Self::SWAPS({
            //         let mut c = SWAPS::new();
            //         c.init_contract_terms(sm_terms.clone());
            //         c.init_risk_factor_external_data(risk_factor_external_data);
            //         c.init_risk_factor_external_event(risk_factor_external_event);
            //         c.init_related_contracts(sm_terms.clone());
            //         c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
            //         c
            //     }))
            //
            // },
            "SWPPV" => {
                Ok(Self::SWPPV({
                    let mut c = SWPPV::new();
                    c.init_contract_terms(sm_terms.clone());
                    c.init_risk_factor_external_data(risk_factor_external_data);
                    c.init_risk_factor_external_event(risk_factor_external_event);
                    c.init_related_contracts(sm_terms.clone());
                    c.init_state_space(&c.contract_terms.maturity_date.clone()); // a voir si c'est ok pour la maturité : pas très sur..
                    c.init_status_date();
                    c
                }))

            },
            // "UMP" => {
            //     Ok(Self::UMP({
            //         let mut c = UMP::new();
            //         c.set_contract_terms(sm_terms);
            //         c.set_contract_risk_factors(risk_factors);
            //         c.set_contract_structure(sm_terms);
            // 
            //         c
            //     }))
            // 
            // },


            _ => Err(format!("Unknown contract type {}", ct))
        }
    }
    
    pub fn run_schedule(&mut self, to: Option<PhantomIsoDatetimeW> ) { // to: Option<IsoDatetime>
        match self {
            ContractModel::ANN(c) => {c.init_contract_event_timeline(to)},
            // ContractModel::BCS(c) => {c.schedule(to)},
            // ContractModel::CAPFL(c) => {c.schedule(to)},
            // ContractModel::CEC(c) => {c.schedule(to)},
            // ContractModel::CEG(c) => {c.schedule(to)},
            // ContractModel::CLM(c) => {c.schedule(to)},
            // ContractModel::COM(c) => {c.schedule(to)},
            // ContractModel::CSH(c) => {c.schedule(to)},
            // ContractModel::FUTUR(c) => {c.schedule(to)},
            ContractModel::FXOUT(c) => {c.init_contract_event_timeline(to)},
            ContractModel::LAM(c) => {c.init_contract_event_timeline(to)},
            // ContractModel::LAX(c) => {c.schedule(to)},
            ContractModel::NAM(c) => {c.init_contract_event_timeline(to)},
            // ContractModel::OPTNS(c) => {c.schedule(to)},
            ContractModel::PAM(c) => {c.init_contract_event_timeline(to)},
            ContractModel::STK(c) => {c.init_contract_event_timeline(to)},
            // ContractModel::SWAPS(c) => {c.init_contract_event_timeline(to)},
            ContractModel::SWPPV(c) => {c.init_contract_event_timeline(to)},
            // ContractModel::UMP(c) => {c.schedule(to)},
        }
    }

    pub fn run_apply(&mut self, stop_states_space_date: Option<PhantomIsoDatetimeW>, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> {
        match self {
            ContractModel::ANN(c) => {
                let it = c.apply_until_date(stop_states_space_date, extract_results);
                it
            },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => {
                let it = c.apply_until_date(stop_states_space_date, extract_results);
                it
            },
            ContractModel::LAM(c) => {
                let it = c.apply_until_date(stop_states_space_date, extract_results);
                it},
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => {
                let it = c.apply_until_date(stop_states_space_date, extract_results);
                it
            },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => {
                let it = c.apply_until_date(stop_states_space_date, extract_results);
                it

            },
            ContractModel::STK(c) => {
                let it = c.apply_until_date(stop_states_space_date, extract_results);
                it
            },
            // ContractModel::SWAPS(c) => {
            //     let it = c.apply_until_date(stop_states_space_date, extract_results);
            //     it
            // },
            ContractModel::SWPPV(c) => {
                let it = c.apply_until_date(stop_states_space_date, extract_results);
                it
            },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }
    
    pub fn next_day(&mut self, extract_results: bool)  -> Option<Result<Vec<TestResult>, String>> {
        match self {
            ContractModel::ANN(c) => {c.next_day(extract_results)},
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => {c.next_day(extract_results)},
            ContractModel::LAM(c) => {c.next_day(extract_results)},
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => {c.next_day(extract_results)},
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => {c.next_day(extract_results)},
            ContractModel::STK(c) => {c.next_day(extract_results)},
            // ContractModel::SWAPS(c) => {c.next()},
            ContractModel::SWPPV(c) => {c.next_day(extract_results)},
            // ContractModel::UMP(c) => {c.apply()},
        }
    }

    pub fn next_event(&mut self, extract_results: bool)  -> Option<Result<TestResult, String>> {
        match self {
            ContractModel::ANN(c) => {c.next_event(extract_results)},
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => {c.next_event(extract_results)},
            ContractModel::LAM(c) => {c.next_event(extract_results)},
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => {c.next_event(extract_results)},
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => {c.next_event(extract_results)},
            ContractModel::STK(c) => {c.next_event(extract_results)},
            // ContractModel::SWAPS(c) => {c.next()},
            ContractModel::SWPPV(c) => {c.next_event(extract_results)},
            // ContractModel::UMP(c) => {c.apply()},
        }
    }

    pub fn get_current_datetime(&self) -> Option<PhantomIsoDatetimeW> {
        match self {
            ContractModel::ANN(c) => {
                let c: PhantomIsoDatetimeW = c.status_date?.convert();
                Some(c)
            },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => {
                let c: PhantomIsoDatetimeW = c.status_date?.convert();
                Some(c)
            },
            ContractModel::LAM(c) => {
                let c: PhantomIsoDatetimeW = c.status_date?.convert();
                Some(c)
            },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => {
                let c: PhantomIsoDatetimeW = c.status_date?.convert();
                Some(c)
            },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => {
                let c: PhantomIsoDatetimeW = c.status_date?.convert();
                Some(c)
            },
            ContractModel::STK(c) => {
                let c: PhantomIsoDatetimeW = c.status_date?.convert();
                Some(c)
            },
            // ContractModel::SWAPS(c) => {
            //     let c: PhantomIsoDatetimeW = c.status_date?.convert();
            //     Some(c)
            // },
            ContractModel::SWPPV(c) => {
                let c: PhantomIsoDatetimeW = c.status_date?.convert();
                Some(c)
            },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }
    
    pub fn get_current_timeline(&self) -> Vec<ContractEvent> {
        match self {
            ContractModel::ANN(c) => {  c.event_timeline.clone()  },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => { c.event_timeline.clone() },
            ContractModel::LAM(c) => { c.event_timeline.clone() },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => { c.event_timeline.clone() },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => { c.event_timeline.clone() },
            ContractModel::STK(c) => { c.event_timeline.clone() },
            // ContractModel::SWAPS(c) => { c.event_timeline.clone() },
            ContractModel::SWPPV(c) => { c.event_timeline.clone() },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }

    pub fn get_current_event_index(&self) -> i32 {
        match self {
            ContractModel::ANN(c) => {  c.curr_event_index },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => { c.curr_event_index },
            ContractModel::LAM(c) => { c.curr_event_index },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => { c.curr_event_index },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => { c.curr_event_index },
            ContractModel::STK(c) => { c.curr_event_index },
            // ContractModel::SWAPS(c) => { c.event_timeline.clone() },
            ContractModel::SWPPV(c) => { c.curr_event_index },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }

    pub fn get_contract_identifiant(&self) -> ContractID {
        match self {
            ContractModel::ANN(c) => {  c.contract_id.clone()  },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => { c.contract_id.clone() },
            ContractModel::LAM(c) => { c.contract_id.clone() },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => { c.contract_id.clone() },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => { c.contract_id.clone() },
            ContractModel::STK(c) => { c.contract_id.clone() },
            // ContractModel::SWAPS(c) => { c.event_timeline.clone() },
            ContractModel::SWPPV(c) => { c.contract_id.clone() },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }

    pub fn get_contract_status_date(&self) -> Option<StatusDate> {
        match self {
            ContractModel::ANN(c) => {  c.status_date.clone()  },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => { c.status_date.clone() },
            ContractModel::LAM(c) => { c.status_date.clone() },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => { c.status_date.clone()},
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => { c.status_date.clone() },
            ContractModel::STK(c) => { c.status_date.clone() },
            // ContractModel::SWAPS(c) => { c.event_timeline.clone() },
            ContractModel::SWPPV(c) => { c.status_date.clone() },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }

    pub fn get_contract_first_last_event_time(&self) -> (Option<PhantomIsoDatetimeW>, Option<PhantomIsoDatetimeW>) {
        match self {
            ContractModel::ANN(c) => {  (c.first_event_date, c.last_event_date)  },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => { (c.first_event_date, c.last_event_date)  },
            ContractModel::LAM(c) => { (c.first_event_date, c.last_event_date)  },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => { (c.first_event_date, c.last_event_date) },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => { (c.first_event_date, c.last_event_date)  },
            ContractModel::STK(c) => { (c.first_event_date, c.last_event_date)  },
            // ContractModel::SWAPS(c) => { c.event_timeline.clone() },
            ContractModel::SWPPV(c) => { (c.first_event_date, c.last_event_date)  },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }
    
    pub fn get_contract_states_space(&self) -> StatesSpace {
        match self {
            ContractModel::ANN(c) => {  c.states_space.clone()  },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => { c.states_space.clone() },
            ContractModel::LAM(c) => { c.states_space.clone() },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => { c.states_space.clone() },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => { c.states_space.clone() },
            ContractModel::STK(c) => { c.states_space.clone() },
            // ContractModel::SWAPS(c) => { c.event_timeline.clone() },
            ContractModel::SWPPV(c) => { c.states_space.clone() },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }
    
    pub fn sort_current_timeline(&mut self) {
        match self {
            ContractModel::ANN(c) => {  c.sort_events_timeline()  },
            // ContractModel::BCS(c) => {c.apply()},
            // ContractModel::CAPFL(c) => {c.apply()},
            // ContractModel::CEC(c) => {c.apply()},
            // ContractModel::CEG(c) => {c.apply()},
            // ContractModel::CLM(c) => {c.apply()},
            // ContractModel::COM(c) => {c.apply()},
            // ContractModel::CSH(c) => {c.apply()},
            // ContractModel::FUTUR(c) => {c.apply()},
            ContractModel::FXOUT(c) => { c.sort_events_timeline() },
            ContractModel::LAM(c) => { c.sort_events_timeline() },
            // ContractModel::LAX(c) => {c.apply()},
            ContractModel::NAM(c) => { c.sort_events_timeline() },
            // ContractModel::OPTNS(c) => {c.apply()},
            ContractModel::PAM(c) => { c.sort_events_timeline()},
            ContractModel::STK(c) => { c.sort_events_timeline() },
            // ContractModel::SWAPS(c) => { c.sort_events_timeline() },
            ContractModel::SWPPV(c) => {c.sort_events_timeline() },
            // ContractModel::UMP(c) => {c.apply()},
        }
    }
    
    pub fn run(&mut self, to: Option<PhantomIsoDatetimeW>, stop_states_space_date: Option<PhantomIsoDatetimeW>, extract_results: bool) {
        self.run_schedule(to);
        self.run_apply(stop_states_space_date, extract_results);
    }

}