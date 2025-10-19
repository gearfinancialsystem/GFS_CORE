#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::swppv::pof::POF_IED_SWPPV::POF_IED_SWPPV;
use crate::functions::swppv::pof::POF_IP_SWPPV::POF_IP_SWPPV;
use crate::functions::swppv::pof::POF_IPFloat_SWPPV::POF_IPFloat_SWPPV;
use crate::functions::swppv::pof::POF_IPFix_SWPPV::POF_IPFix_SWPPV;
use crate::functions::swppv::pof::POF_MD_SWPPV::POF_MD_SWPPV;
use crate::functions::swppv::pof::POF_PRD_SWPPV::POF_PRD_SWPPV;
use crate::functions::swppv::pof::POF_RR_SWPPV::POF_RR_SWPPV;
use crate::functions::swppv::pof::POF_TD_SWPPV::POF_TD_SWPPV;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;

#[derive(Clone)]
pub enum PayOffFunctionSWPPV {
    // POF_AD_SWPPV(POF_AD_SWPPV),
    // POF_CE_SWPPV(POF_CE_SWPPV),
    POF_IED_SWPPV(POF_IED_SWPPV),
    POF_IP_SWPPV(POF_IP_SWPPV),
    POF_IPFix_SWPPV(POF_IPFix_SWPPV),
    POF_IPFloat_SWPPV(POF_IPFloat_SWPPV),
    POF_MD_SWPPV(POF_MD_SWPPV),
    POF_PRD_SWPPV(POF_PRD_SWPPV),
    POF_RR_SWPPV(POF_RR_SWPPV),
    POF_TD_SWPPV(POF_TD_SWPPV),
}


impl PayOffFunctionSWPPV {
    pub fn from_str(func: &str) -> PayOffFunctionSWPPV     {
        match func {
            // "POF_AD_SWPPV" => Self::POF_AD_SWPPV(POF_AD_SWPPV::new()),
            // "POF_CE_SWPPV" => Self::POF_CE_SWPPV(POF_AD_SWPPV::new()),
            "POF_IED_SWPPV" => Self::POF_IED_SWPPV(POF_IED_SWPPV::new()),
            "POF_IP_SWPPV" => Self::POF_IP_SWPPV(POF_IP_SWPPV::new()),
            "POF_IPFix_SWPPV" => Self::POF_IPFix_SWPPV(POF_IPFix_SWPPV::new()),
            "POF_IPFloat_SWPPV" => Self::POF_IPFloat_SWPPV(POF_IPFloat_SWPPV::new()),
            "POF_MD_SWPPV" => Self::POF_MD_SWPPV(POF_MD_SWPPV::new()),
            "POF_PRD_SWPPV" => Self::POF_PRD_SWPPV(POF_PRD_SWPPV::new()),
            "POF_RR_SWPPV" => Self::POF_RR_SWPPV(POF_RR_SWPPV::new()),
            "POF_TD_SWPPV" => Self::POF_TD_SWPPV(POF_TD_SWPPV::new()),
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
            // PayOffFunctionSWPPV::POF_AD_SWPPV(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            // PayOffFunctionSWPPV::POF_CE_SWPPV(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            PayOffFunctionSWPPV::POF_IED_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWPPV::POF_IP_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWPPV::POF_IPFix_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWPPV::POF_IPFloat_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWPPV::POF_MD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWPPV::POF_PRD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWPPV::POF_RR_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWPPV::POF_TD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}
