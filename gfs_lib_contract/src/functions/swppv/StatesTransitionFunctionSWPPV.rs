#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::swppv::stf::STF_AD_SWPPV::STF_AD_SWPPV;
use crate::functions::swppv::stf::STF_CD_SWPPV::STF_CD_SWPPV;
use crate::functions::swppv::stf::STF_IED_SWPPV::STF_IED_SWPPV;
use crate::functions::swppv::stf::STF_IP_SWPPV::STF_IP_SWPPV;
use crate::functions::swppv::stf::STF_IPFix_SWPPV::STF_IPFix_SWPPV;
use crate::functions::swppv::stf::STF_IPFloat_SWPPV::STF_IPFloat_SWPPV;
use crate::functions::swppv::stf::STF_MD_SWPPV::STF_MD_SWPPV;
use crate::functions::swppv::stf::STF_PRD_SWPPV::STF_PRD_SWPPV;
use crate::functions::swppv::stf::STF_RR_SWPPV::STF_RR_SWPPV;
use crate::functions::swppv::stf::STF_TD_SWPPV::STF_TD_SWPPV;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionSWPPV {

    STF_AD_SWPPV(STF_AD_SWPPV),
    STF_CD_SWPPV(STF_CD_SWPPV),
    // STF_CE_SWPPV(STF_CE_SWPPV),
    STF_IED_SWPPV(STF_IED_SWPPV),
    STF_IP_SWPPV(STF_IP_SWPPV),
    STF_IPFix_SWPPV(STF_IPFix_SWPPV),
    STF_IPFloat_SWPPV(STF_IPFloat_SWPPV),
    STF_MD_SWPPV(STF_MD_SWPPV),
    STF_PRD_SWPPV(STF_PRD_SWPPV),
    STF_RR_SWPPV(STF_RR_SWPPV),
    STF_TD_SWPPV(STF_TD_SWPPV),
}

impl StatesTransitionFunctionSWPPV {
    pub fn from_str(func: &str) -> StatesTransitionFunctionSWPPV {
        match func {
            "STF_AD_SWPPV" => Self::STF_AD_SWPPV(STF_AD_SWPPV::new()),
            "STF_CD_SWPPV" => Self::STF_CD_SWPPV(STF_CD_SWPPV::new()),
            // "STF_CE_SWPPV" => Self::STF_CE_SWPPV(STF_CE_SWPPV::new()),
            "STF_IED_SWPPV" => Self::STF_IED_SWPPV(STF_IED_SWPPV::new()),
            "STF_IP_SWPPV" => Self::STF_IP_SWPPV(STF_IP_SWPPV::new()),
            "STF_IPFix_SWPPV" => Self::STF_IPFix_SWPPV(STF_IPFix_SWPPV::new()),
            "STF_IPFloat_SWPPV" => Self::STF_IPFloat_SWPPV(STF_IPFloat_SWPPV::new()),
            "STF_MD_SWPPV" => Self::STF_MD_SWPPV(STF_MD_SWPPV::new()),
            "STF_PRD_SWPPV" => Self::STF_PRD_SWPPV(STF_PRD_SWPPV::new()),
            "STF_RR_SWPPV" => Self::STF_RR_SWPPV(STF_RR_SWPPV::new()),
            "STF_TD_SWPPV" => Self::STF_TD_SWPPV(STF_TD_SWPPV::new()),
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
            StatesTransitionFunctionSWPPV::STF_AD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_CD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // StatesTransitionFunctionSWPPV::STF_CE_SWPPV(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            StatesTransitionFunctionSWPPV::STF_IED_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_IP_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_IPFix_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_IPFloat_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_MD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_PRD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_RR_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWPPV::STF_TD_SWPPV(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}