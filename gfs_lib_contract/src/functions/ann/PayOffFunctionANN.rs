#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::functions::ann::pof::POF_AD_ANN::POF_AD_ANN;
use crate::functions::ann::pof::POF_CE_ANN::POF_CE_ANN;
use crate::functions::ann::pof::POF_FP_ANN::POF_FP_ANN;
use crate::functions::ann::pof::POF_IED_ANN::POF_IED_ANN;
use crate::functions::ann::pof::POF_IP_ANN::POF_IP_ANN;
use crate::functions::ann::pof::POF_IPCB_ANN::POF_IPCB_ANN;
use crate::functions::ann::pof::POF_IPCI_ANN::POF_IPCI_ANN;
use crate::functions::ann::pof::POF_MD_ANN::POF_MD_ANN;
use crate::functions::ann::pof::POF_PP_ANN::POF_PP_ANN;
use crate::functions::ann::pof::POF_PR_ANN::POF_PR_ANN;
use crate::functions::ann::pof::POF_PRD_ANN::POF_PRD_ANN;
use crate::functions::ann::pof::POF_PY_ANN::POF_PY_ANN;
use crate::functions::ann::pof::POF_RR_ANN::POF_RR_ANN;
use crate::functions::ann::pof::POF_SC_ANN::POF_SC_ANN;
use crate::functions::ann::pof::POF_TD_ANN::POF_TD_ANN;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;


#[derive(Clone)]
pub enum PayOffFunctionANN {
    POF_AD_ANN(POF_AD_ANN),
    POF_CE_ANN(POF_CE_ANN),
    POF_FP_ANN(POF_FP_ANN),
    POF_IED_ANN(POF_IED_ANN),
    POF_IP_ANN(POF_IP_ANN),
    POF_IPCB_ANN(POF_IPCB_ANN),
    POF_IPCI_ANN(POF_IPCI_ANN),
    POF_MD_ANN(POF_MD_ANN),
    POF_PP_ANN(POF_PP_ANN),
    POF_PR_ANN(POF_PR_ANN),
    POF_PRD_ANN(POF_PRD_ANN),
    POF_PY_ANN(POF_PY_ANN),
    POF_RR_ANN(POF_RR_ANN),
    // POF_RRY_ANN(POF_RRY_ANN),
    POF_SC_ANN(POF_SC_ANN),
    POF_TD_ANN(POF_TD_ANN),
}


impl PayOffFunctionANN {
    pub fn from_str(func: &str) -> PayOffFunctionANN    {
        match func {
            "POF_AD_ANN" => Self::POF_AD_ANN(POF_AD_ANN::new()),
            "POF_CE_ANN" => Self::POF_CE_ANN(POF_CE_ANN::new()),
            "POF_FP_ANN" => Self::POF_FP_ANN(POF_FP_ANN::new()),
            "POF_IED_ANN" => Self::POF_IED_ANN(POF_IED_ANN::new()),
            "POF_IP_ANN" => Self::POF_IP_ANN(POF_IP_ANN::new()),
            "POF_IPCI_ANN" => Self::POF_IPCI_ANN(POF_IPCI_ANN::new()),
            "POF_MD_ANN" => Self::POF_MD_ANN(POF_MD_ANN::new()),
            "POF_PP_ANN" => Self::POF_PP_ANN(POF_PP_ANN::new()),
            "POF_PR_ANN" => Self::POF_PR_ANN(POF_PR_ANN::new()),
            "POF_PRD_ANN" => Self::POF_PRD_ANN(POF_PRD_ANN::new()),
            "POF_PY_ANN" => Self::POF_PY_ANN(POF_PY_ANN::new()),
            "POF_RR_ANN" => Self::POF_RR_ANN(POF_RR_ANN::new()),
            //"POF_RRY_ANN" => Self::POF_RRY_ANN(POF_RRY_ANN::new()),
            "POF_SC_ANN" => Self::POF_SC_ANN(POF_SC_ANN::new()),
            "POF_TD_ANN" => Self::POF_TD_ANN(POF_TD_ANN::new()),
            _ => panic!("Unknown function {}", func)
        }
    }



    pub fn eval(&self,
            time: &PhantomIsoDatetimeW,
            states: &StatesSpace,
            contract_terms: &ContractTerms,
            contract_structure: &Option<RelatedContracts>,
            risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
            day_counter: &Option<DayCountConvention>,
            time_adjuster: &BusinessDayAdjuster,
        ) -> Result<PayOff, ErrorContractEnum> {
        match self {
            PayOffFunctionANN::POF_AD_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_CE_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_FP_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_IED_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_IP_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_IPCB_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_IPCI_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_MD_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_PP_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_PR_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_PRD_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_PY_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_RR_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // PayOffFunctionANN::POF_RRF_ANN(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            // PayOffFunctionANN::POF_RRY_ANN(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            PayOffFunctionANN::POF_SC_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionANN::POF_TD_ANN(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}
