#![allow(non_camel_case_types)]

use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::stk::pof::POF_AD_STK::POF_AD_STK;
use crate::functions::stk::pof::POF_CE_STK::POF_CE_STK;
use crate::functions::stk::pof::POF_DV_STK::POF_DV_STK;
use crate::functions::stk::pof::POF_PRD_STK::POF_PRD_STK;
use crate::functions::stk::pof::POF_TD_STK::POF_TD_STK;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;


#[derive(Clone)]
pub enum PayOffFunctionSTK {
    POF_AD_STK(POF_AD_STK),
    // POF_CD_STK(POF_CD_STK),
    POF_CE_STK(POF_CE_STK),
    POF_DV_STK(POF_DV_STK),
    POF_PRD_STK(POF_PRD_STK),
    POF_TD_STK(POF_TD_STK),
}


impl PayOffFunctionSTK {
    pub fn from_str(func: &str) -> PayOffFunctionSTK     {
        match func {
            "POF_AD_STK" => Self::POF_AD_STK(POF_AD_STK::new()),
            // "POF_CD_STK" => Self::POF_CD_STK(POF_CD_STK::new()),
            "POF_CE_STK" => Self::POF_CE_STK(POF_CE_STK::new()),
            "POF_DV_STK" => Self::POF_DV_STK(POF_DV_STK::new()),
            "POF_PRD_STK" => Self::POF_PRD_STK(POF_PRD_STK::new()),
            "POF_TD_STK" => Self::POF_TD_STK(POF_TD_STK::new()),
            _ => panic!("Unknown function {}", func)
        }
    }

    pub fn eval(&self,
            time: &PhantomIsoDatetimeW,
            states: &StatesSpace,
            contract_terms: &ContractTerms,
            contract_structure: &Option<RelatedContracts>,
            risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
            day_counter: &Option<DayCountConvention>,
            time_adjuster: &BusinessDayAdjuster,
        ) -> f64 {
        match self {
            PayOffFunctionSTK::POF_AD_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // PayOffFunctionFXOUT::POF_CD_STK(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            PayOffFunctionSTK::POF_CE_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSTK::POF_DV_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSTK::POF_PRD_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionSTK::POF_TD_STK(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}
