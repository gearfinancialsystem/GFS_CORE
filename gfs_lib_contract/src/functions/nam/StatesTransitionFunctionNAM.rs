#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::nam::stf::STF_AD_NAM::STF_AD_NAM;
use crate::functions::nam::stf::STF_CE_NAM::STF_CE_NAM;
use crate::functions::nam::stf::STF_IED_NAM::STF_IED_NAM;
use crate::functions::nam::stf::STF_IP_NAM::STF_IP_NAM;
use crate::functions::nam::stf::STF_IPCB_NAM::STF_IPCB_NAM;
use crate::functions::nam::stf::STF_IPCI_NAM::STF_IPCI_NAM;
use crate::functions::nam::stf::STF_MD_NAM::STF_MD_NAM;
use crate::functions::nam::stf::STF_PP_NAM::STF_PP_NAM;
use crate::functions::nam::stf::STF_PR2_NAM::STF_PR2_NAM;
use crate::functions::nam::stf::STF_PR_NAM::STF_PR_NAM;
use crate::functions::nam::stf::STF_PRD_NAM::STF_PRD_NAM;
use crate::functions::nam::stf::STF_PY_NAM::STF_PY_NAM;
use crate::functions::nam::stf::STF_RR_NAM::STF_RR_NAM;
use crate::functions::nam::stf::STF_RRF_NAM::STF_RRF_NAM;
use crate::functions::nam::stf::STF_SC_NAM::STF_SC_NAM;
use crate::functions::nam::stf::STF_TD_NAM::STF_TD_NAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionNAM {
    STF_AD_NAM(STF_AD_NAM),
    STF_CE_NAM(STF_CE_NAM),
    // STF_FP_NAM(STF_FP_NAM),
    STF_IED_NAM(STF_IED_NAM),
    STF_IP_NAM(STF_IP_NAM),
    STF_IPCB_NAM(STF_IPCB_NAM),
    STF_IPCI_NAM(STF_IPCI_NAM),
    STF_MD_NAM(STF_MD_NAM),
    STF_PP_NAM(STF_PP_NAM),
    STF_PR_NAM(STF_PR_NAM),
    STF_PR2_NAM(STF_PR2_NAM),
    STF_PRD_NAM(STF_PRD_NAM),
    STF_PY_NAM(STF_PY_NAM),
    STF_RR_NAM(STF_RR_NAM),
    STF_RRF_NAM(STF_RRF_NAM),
    STF_SC_NAM(STF_SC_NAM),
    STF_TD_NAM(STF_TD_NAM),
}

impl StatesTransitionFunctionNAM {
    pub fn from_str(func: &str) -> StatesTransitionFunctionNAM {
        match func {
            "STF_AD_NAM" => Self::STF_AD_NAM(STF_AD_NAM::new()),
            "STF_CE_NAM" => Self::STF_CE_NAM(STF_CE_NAM::new()),
            //"STF_FP_NAM" => Self::STF_FP_NAM(STF_FP_NAM::new()),
            "STF_IED_NAM" => Self::STF_IED_NAM(STF_IED_NAM::new()),
            "STF_IP_NAM" => Self::STF_IP_NAM(STF_IP_NAM::new()),
            "STF_IPCB_NAM" => Self::STF_IPCB_NAM(STF_IPCB_NAM::new()),
            "STF_IPCI_NAM" => Self::STF_IPCI_NAM(STF_IPCI_NAM::new()),
            "STF_MD_NAM" => Self::STF_MD_NAM(STF_MD_NAM::new()),
            "STF_PP_NAM" => Self::STF_PP_NAM(STF_PP_NAM::new()),
            "STF_PR_NAM" => Self::STF_PR_NAM(STF_PR_NAM::new()),
            "STF_PR2_NAM" => Self::STF_PR2_NAM(STF_PR2_NAM::new()),
            "STF_PRD_NAM" => Self::STF_PRD_NAM(STF_PRD_NAM::new()),
            "STF_PY_NAM" => Self::STF_PY_NAM(STF_PY_NAM::new()),
            "STF_RR_NAM" => Self::STF_RR_NAM(STF_RR_NAM::new()),
            "STF_RRF_NAM" => Self::STF_RRF_NAM(STF_RRF_NAM::new()),
            "STF_SC_NAM" => Self::STF_SC_NAM(STF_SC_NAM::new()),
            "STF_TD_NAM" => Self::STF_TD_NAM(STF_TD_NAM::new()),
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
            StatesTransitionFunctionNAM::STF_AD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_CE_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // StatesTransitionFunctionNAM::STF_FP_NAM(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            StatesTransitionFunctionNAM::STF_IED_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_IP_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_IPCB_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_IPCI_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_MD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_PP_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_PR_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_PR2_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_PRD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_PY_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_RR_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_RRF_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_SC_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionNAM::STF_TD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}