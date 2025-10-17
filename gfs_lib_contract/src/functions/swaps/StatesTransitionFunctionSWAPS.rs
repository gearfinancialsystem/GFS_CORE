#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::swaps::stf::STF_NET_SWAPS::STF_NET_SWAPS;
// use crate::functions::swaps::stf::STF_NET_SWAPS::STF_NET_SWAPS;
use crate::functions::swaps::stf::STF_PRD_SWAPS::STF_PRD_SWAPS;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionSWAPS {

    // STF_CD_SWAPS(STF_AD_SWAPS),
    // STF_CE_SWAPS(STF_CE_SWAPS),
    STF_NET_SWAPS(STF_NET_SWAPS),
    STF_PRD_SWAPS(STF_PRD_SWAPS),
    // STF_TD_SWAPS(STF_TD_SWAPS),
}

impl StatesTransitionFunctionSWAPS {
    pub fn from_str(func: &str) -> StatesTransitionFunctionSWAPS {
        match func {
            // "STF_AD_SWAPS" => Self::STF_AD_SWAPS(STF_AD_SWAPS::new()),
            // "STF_CE_SWAPS" => Self::STF_CE_SWAPS(STF_CE_SWAPS::new()),
            "STF_NET_SWAPS" => Self::STF_NET_SWAPS(STF_NET_SWAPS::new()),
            "STF_PRD_SWAPS" => Self::STF_PRD_SWAPS(STF_PRD_SWAPS::new()),
            // "STF_TD_SWAPS" => Self::STF_TD_SWAPS(STF_TD_SWAPS::new()),
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
            // StatesTransitionFunctionSWAPS::STF_AD_SWAPS(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            // StatesTransitionFunctionSWAPS::STF_CE_SWAPS(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            StatesTransitionFunctionSWAPS::STF_NET_SWAPS(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSWAPS::STF_PRD_SWAPS(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // StatesTransitionFunctionSWAPS::STF_TD_SWAPS(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
        }
    }
}