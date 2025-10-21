#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::ann::stf::STF_AD_ANN::STF_AD_ANN;
use crate::functions::ann::stf::STF_CE_ANN::STF_CE_ANN;
use crate::functions::ann::stf::STF_FP_ANN::STF_FP_ANN;
use crate::functions::ann::stf::STF_IED_ANN::STF_IED_ANN;
use crate::functions::ann::stf::STF_IP_ANN::STF_IP_ANN;
use crate::functions::ann::stf::STF_IPCB_ANN::STF_IPCB_ANN;
use crate::functions::ann::stf::STF_IPCI_ANN::STF_IPCI_ANN;
use crate::functions::ann::stf::STF_MD_ANN::STF_MD_ANN;
use crate::functions::ann::stf::STF_PP_ANN::STF_PP_ANN;
use crate::functions::ann::stf::STF_PR_ANN::STF_PR_ANN;
use crate::functions::ann::stf::STF_PRF_ANN::STF_PRF_ANN;
use crate::functions::ann::stf::STF_PY_ANN::STF_PY_ANN;
use crate::functions::ann::stf::STF_SC_ANN::STF_SC_ANN;
use crate::functions::ann::stf::STF_TD_ANN::STF_TD_ANN;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionANN {

    STF_AD_ANN(STF_AD_ANN),
    // STF_CD_ANN(STF_CD_ANN),
    STF_CE_ANN(STF_CE_ANN),
    STF_FP_ANN(STF_FP_ANN),
    STF_IED_ANN(STF_IED_ANN),
    STF_IP_ANN(STF_IP_ANN),
    STF_IPCB_ANN(STF_IPCB_ANN),
    STF_IPCI_ANN(STF_IPCI_ANN),
    STF_MD_ANN(STF_MD_ANN),
    STF_PP_ANN(STF_PP_ANN),
    STF_PR_ANN(STF_PR_ANN),
    STF_PRF_ANN(STF_PRF_ANN),
    STF_PY_ANN(STF_PY_ANN),
    //STF_RR_ANN(STF_RR_ANN),
    //STF_RRY_ANN(STF_RRY_ANN),
    STF_SC_ANN(STF_SC_ANN),
    STF_TD_ANN(STF_TD_ANN),
}

impl StatesTransitionFunctionANN {
    pub fn from_str(func: &str) -> StatesTransitionFunctionANN {
        match func {
            "STF_AD_ANN" => Self::STF_AD_ANN(STF_AD_ANN::new()),
            //"STF_CD_ANN" => Self::STF_CD_ANN(STF_CD_ANN::new()),
            "STF_CE_ANN" => Self::STF_CE_ANN(STF_CE_ANN::new()),
            "STF_FP_ANN" => Self::STF_FP_ANN(STF_FP_ANN::new()),
            "STF_IED_ANN" => Self::STF_IED_ANN(STF_IED_ANN::new()),
            "STF_IP_ANN" => Self::STF_IP_ANN(STF_IP_ANN::new()),
            "STF_IPCB_ANN" => Self::STF_IPCB_ANN(STF_IPCB_ANN::new()),
            "STF_IPCI_ANN" => Self::STF_IPCI_ANN(STF_IPCI_ANN::new()),
            "STF_MD_ANN" => Self::STF_MD_ANN(STF_MD_ANN::new()),
            "STF_PP_ANN" => Self::STF_PP_ANN(STF_PP_ANN::new()),
            "STF_PR_ANN" => Self::STF_PR_ANN(STF_PR_ANN::new()),
            "STF_PRF_ANN" => Self::STF_PRF_ANN(STF_PRF_ANN::new()),
            "STF_PY_ANN" => Self::STF_PY_ANN(STF_PY_ANN::new()),
            //"STF_RR_ANN" => Self::STF_RR_ANN(STF_RR_ANN::new()),
            //"STF_RRF_ANN" => Self::STF_RRF_ANN(STF_RRF_ANN::new()),
            //"STF_RRY_ANN" => Self::STF_RRY_ANN(STF_RRY_ANN::new()),
            "STF_SC_ANN" => Self::STF_SC_ANN(STF_SC_ANN::new()),
            "STF_TD_ANN" => Self::STF_TD_ANN(STF_TD_ANN::new()),
            _ => panic!("Unknown function {}", func)
        }
    }
    pub fn eval(&self,
                time: &PhantomIsoDatetimeW,
                states: &mut StatesSpace,
                contract_terms: &ContractTerms,
                contract_structure: &Option<RelatedContracts>,
                risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
                day_counter: &Option<DayCountConvention>,
                time_adjuster: &BusinessDayAdjuster,
    ) {
        match self {
            StatesTransitionFunctionANN::STF_AD_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // StatesTransitionFunctionANN::STF_CD_ANN(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            StatesTransitionFunctionANN::STF_CE_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_FP_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_IED_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_IP_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_IPCB_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_IPCI_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_MD_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_PP_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_PR_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_PRF_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_PY_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // StatesTransitionFunctionANN::STF_RR_ANN(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            // StatesTransitionFunctionANN::STF_RRF_ANN(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            // StatesTransitionFunctionANN::STF_RRY_ANN(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            StatesTransitionFunctionANN::STF_SC_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionANN::STF_TD_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}