#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::error::ErrorContract::ErrorContractEnum;
use crate::functions::nam::pof::POF_AD_NAM::POF_AD_NAM;
use crate::functions::nam::pof::POF_CE_NAM::POF_CE_NAM;
use crate::functions::nam::pof::POF_FP_NAM::POF_FP_NAM;
use crate::functions::nam::pof::POF_IED_NAM::POF_IED_NAM;
use crate::functions::nam::pof::POF_IP_NAM::POF_IP_NAM;
use crate::functions::nam::pof::POF_IPCB_NAM::POF_IPCB_NAM;
use crate::functions::nam::pof::POF_IPCI_NAM::POF_IPCI_NAM;
use crate::functions::nam::pof::POF_MD_NAM::POF_MD_NAM;
use crate::functions::nam::pof::POF_PP_NAM::POF_PP_NAM;
use crate::functions::nam::pof::POF_PR_NAM::POF_PR_NAM;
use crate::functions::nam::pof::POF_PRD_NAM::POF_PRD_NAM;
use crate::functions::nam::pof::POF_PY_NAM::POF_PY_NAM;
use crate::functions::nam::pof::POF_RR_NAM::POF_RR_NAM;
use crate::functions::nam::pof::POF_RRF_NAM::POF_RRF_NAM;
use crate::functions::nam::pof::POF_SC_NAM::POF_SC_NAM;
use crate::functions::nam::pof::POF_TD_NAM::POF_TD_NAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;


#[derive(Clone)]
pub enum PayOffFunctionNAM {
    POF_AD_NAM(POF_AD_NAM),
    POF_CE_NAM(POF_CE_NAM),
    POF_FP_NAM(POF_FP_NAM),
    POF_IED_NAM(POF_IED_NAM),
    POF_IP_NAM(POF_IP_NAM),
    POF_IPCB_NAM(POF_IPCB_NAM),
    POF_IPCI_NAM(POF_IPCI_NAM),
    POF_MD_NAM(POF_MD_NAM),
    POF_PP_NAM(POF_PP_NAM),
    POF_PR_NAM(POF_PR_NAM),
    POF_PRD_NAM(POF_PRD_NAM),
    POF_PY_NAM(POF_PY_NAM),
    POF_RR_NAM(POF_RR_NAM),
    POF_RRF_NAM(POF_RRF_NAM),
    POF_SC_NAM(POF_SC_NAM),
    POF_TD_NAM(POF_TD_NAM),
}


impl PayOffFunctionNAM {
    pub fn from_str(func: &str) -> PayOffFunctionNAM {
        match func {
            "POF_AD_NAM" => Self::POF_AD_NAM(POF_AD_NAM::new()),
            "POF_CE_NAM" => Self::POF_CE_NAM(POF_CE_NAM::new()),
            "POF_FP_NAM" => Self::POF_FP_NAM(POF_FP_NAM::new()),
            "POF_IED_NAM" => Self::POF_IED_NAM(POF_IED_NAM::new()),
            "POF_IP_NAM" => Self::POF_IP_NAM(POF_IP_NAM::new()),
            "POF_IPCB_NAM" => Self::POF_IPCB_NAM(POF_IPCB_NAM::new()),
            "POF_IPCI_NAM" => Self::POF_IPCI_NAM(POF_IPCI_NAM::new()),
            "POF_MD_NAM" => Self::POF_MD_NAM(POF_MD_NAM::new()),
            "POF_PP_NAM" => Self::POF_PP_NAM(POF_PP_NAM::new()),
            "POF_PR_NAM" => Self::POF_PR_NAM(POF_PR_NAM::new()),
            "POF_PRD_NAM" => Self::POF_PRD_NAM(POF_PRD_NAM::new()),
            "POF_PY_NAM" => Self::POF_PY_NAM(POF_PY_NAM::new()),
            "POF_RR_NAM" => Self::POF_RR_NAM(POF_RR_NAM::new()),
            "POF_RRF_NAM" => Self::POF_RRF_NAM(POF_RRF_NAM::new()),
            "POF_SC_NAM" => Self::POF_SC_NAM(POF_SC_NAM::new()),
            "POF_TD_NAM" => Self::POF_TD_NAM(POF_TD_NAM::new()),
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
            PayOffFunctionNAM::POF_AD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_CE_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_FP_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_IED_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_IP_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_IPCB_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_IPCI_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_MD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_PP_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_PR_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_PRD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_PY_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_RR_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_RRF_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_SC_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionNAM::POF_TD_NAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}
