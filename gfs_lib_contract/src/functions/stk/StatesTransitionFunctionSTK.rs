#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::stk::stf::STF_CD_STK::STF_CD_STK;
use crate::functions::stk::stf::STF_DV_STK::STF_DV_STK;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionSTK {

    STF_CD_STK(STF_CD_STK),
    // STF_CE_STK(STF_CE_STK),
    STF_DV_STK(STF_DV_STK),
    STF_PRD_STK(STF_PRD_STK),
    STF_TD_STK(STF_TD_STK),
}

impl StatesTransitionFunctionSTK {
    pub fn from_str(func: &str) -> StatesTransitionFunctionSTK {
        match func {
            "STF_CD_STK" => Self::STF_CD_STK(STF_CD_STK::new()),
            // "STF_CE_STK" => Self::STF_CE_STK(STF_CE_STK::new()),
            "STF_DV_STK" => Self::STF_DV_STK(STF_DV_STK::new()),
            "STF_PRD_STK" => Self::STF_PRD_STK(STF_PRD_STK::new()),
            "STF_TD_STK" => Self::STF_TD_STK(STF_TD_STK::new()),
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
            StatesTransitionFunctionSTK::STF_CD_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // StatesTransitionFunctionSTK::STF_CE_STK(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            StatesTransitionFunctionSTK::STF_DV_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSTK::STF_PRD_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionSTK::STF_TD_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}