#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::lam::pof::POF_AD_LAM::POF_AD_LAM;
use crate::functions::lam::pof::POF_CE_LAM::POF_CE_LAM;
use crate::functions::lam::pof::POF_FP_LAM::POF_FP_LAM;
use crate::functions::lam::pof::POF_IED_LAM::POF_IED_LAM;
use crate::functions::lam::pof::POF_IP_LAM::POF_IP_LAM;
use crate::functions::lam::pof::POF_IPCI_LAM::POF_IPCI_LAM;
use crate::functions::lam::pof::POF_MD_LAM::POF_MD_LAM;
use crate::functions::lam::pof::POF_PP_LAM::POF_PP_LAM;
use crate::functions::lam::pof::POF_PR_LAM::POF_PR_LAM;
use crate::functions::lam::pof::POF_PRD_LAM::POF_PRD_LAM;
use crate::functions::lam::pof::POF_PY_LAM::POF_PY_LAM;
use crate::functions::lam::pof::POF_RR_LAM::POF_RR_LAM;
use crate::functions::lam::pof::POF_RRF_LAM::POF_RRF_LAM;
use crate::functions::lam::pof::POF_SC_LAM::POF_SC_LAM;
use crate::functions::lam::pof::POF_TD_LAM::POF_TD_LAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;

#[derive(Clone)]
pub enum PayOffFunctionLAM {
    POF_AD_LAM(POF_AD_LAM),
    POF_CE_LAM(POF_CE_LAM),
    POF_FP_LAM(POF_FP_LAM),
    POF_IED_LAM(POF_IED_LAM),
    POF_IP_LAM(POF_IP_LAM),
    POF_IPCI_LAM(POF_IPCI_LAM),
    POF_MD_LAM(POF_MD_LAM),
    POF_PP_LAM(POF_PP_LAM),
    POF_PR_LAM(POF_PR_LAM),
    POF_PRD_LAM(POF_PRD_LAM),
    POF_PY_LAM(POF_PY_LAM),
    POF_RR_LAM(POF_RR_LAM),
    POF_RRF_LAM(POF_RRF_LAM),
    POF_SC_LAM(POF_SC_LAM),
    POF_TD_LAM(POF_TD_LAM),
}

impl PayOffFunctionLAM {
    pub fn from_str(func: &str) -> PayOffFunctionLAM {
        match func {
            "POF_AD_LAM" => Self::POF_AD_LAM(POF_AD_LAM::new()),
            "POF_CE_LAM" => Self::POF_CE_LAM(POF_CE_LAM::new()),
            "POF_FP_LAM" => Self::POF_FP_LAM(POF_FP_LAM::new()),
            "POF_IED_LAM" => Self::POF_IED_LAM(POF_IED_LAM::new()),
            "POF_IP_LAM" => Self::POF_IP_LAM(POF_IP_LAM::new()),
            "POF_IPCI_LAM" => Self::POF_IPCI_LAM(POF_IPCI_LAM::new()),
            "POF_MD_LAM" => Self::POF_MD_LAM(POF_MD_LAM::new()),
            "POF_PP_LAM" => Self::POF_PP_LAM(POF_PP_LAM::new()),
            "POF_PRD_LAM" => Self::POF_PRD_LAM(POF_PRD_LAM::new()),
            "POF_PR_LAM" => Self::POF_PR_LAM(POF_PR_LAM::new()),
            "POF_PY_LAM" => Self::POF_PY_LAM(POF_PY_LAM::new()),
            "POF_RR_LAM" => Self::POF_RR_LAM(POF_RR_LAM::new()),
            "POF_RRF_LAM" => Self::POF_RRF_LAM(POF_RRF_LAM::new()),
            "POF_SC_LAM" => Self::POF_SC_LAM(POF_SC_LAM::new()),
            "POF_TD_LAM" => Self::POF_TD_LAM(POF_TD_LAM::new()),
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
    ) -> f64 {
        match self {
            PayOffFunctionLAM::POF_AD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),

            PayOffFunctionLAM::POF_CE_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_FP_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_IED_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_IP_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_IPCI_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_MD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_PP_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_PRD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_PR_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_PY_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_RR_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_RRF_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_SC_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionLAM::POF_TD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),


        }
    }
}