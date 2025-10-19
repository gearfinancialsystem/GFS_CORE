#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::swaps::pof::POF_AD_SWAPS::POF_AD_SWAPS;
use crate::functions::swaps::pof::POF_NET_SWAPS::POF_NET_SWAPS;
use crate::functions::swaps::pof::POF_PRD_SWAPS::POF_PRD_SWAPS;
use crate::functions::swaps::pof::POF_TD_SWAPS::POF_TD_SWAPS;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;


#[derive(Clone)]
pub enum PayOffFunctionSWAPS {
    POF_AD_SWAPS(POF_AD_SWAPS),
    // POF_CE_SWAPS(POF_CE_SWAPS),
    POF_NET_SWAPS(POF_NET_SWAPS),
    POF_PRD_SWAPS(POF_PRD_SWAPS),
    POF_TD_SWAPS(POF_TD_SWAPS),
}


impl PayOffFunctionSWAPS {
    pub fn from_str(func: &str) -> PayOffFunctionSWAPS     {
        match func {
            "POF_AD_SWAPS" => Self::POF_AD_SWAPS(POF_AD_SWAPS::new()),
            // "POF_CE_SWAPS" => Self::POF_CE_SWAPS(POF_CE_SWAPS::new()),
            "POF_NET_SWAPS" => Self::POF_NET_SWAPS(POF_NET_SWAPS::new()),
            "POF_PRD_SWAPS" => Self::POF_PRD_SWAPS(POF_PRD_SWAPS::new()),
            "POF_TD_SWAPS" => Self::POF_TD_SWAPS(POF_TD_SWAPS::new()),
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
            PayOffFunctionSWAPS::POF_AD_SWAPS(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // PayOffFunctionSWAPS::POF_CE_SWAPS(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            PayOffFunctionSWAPS::POF_NET_SWAPS(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWAPS::POF_PRD_SWAPS(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSWAPS::POF_TD_SWAPS(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}
