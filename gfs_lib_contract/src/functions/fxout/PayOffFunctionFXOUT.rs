#![allow(non_camel_case_types)]

use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::fxout::pof::POF_AD_FXOUT::POF_AD_FXOUT;
use crate::functions::fxout::pof::POF_CD_FXOUT;
use crate::functions::fxout::pof::POF_CE_FXOUT::POF_CE_FXOUT;
use crate::functions::fxout::pof::POF_MD1_FXOUT::POF_MD1_FXOUT;
use crate::functions::fxout::pof::POF_MD2_FXOUT::POF_MD2_FXOUT;
use crate::functions::fxout::pof::POF_PRD_FXOUT::POF_PRD_FXOUT;
use crate::functions::fxout::pof::POF_STD_FXOUT::POF_STD_FXOUT;
use crate::functions::fxout::pof::POF_TD_FXOUT::POF_TD_FXOUT;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;


#[derive(Clone)]
pub enum PayOffFunctionFXOUT {
    POF_AD_FXOUT(POF_AD_FXOUT),
    // POF_CD_FXOUT(POF_CD_FXOUT),
    POF_CE_FXOUT(POF_CE_FXOUT),
    POF_MD1_FXOUT(POF_MD1_FXOUT),
    POF_MD2_FXOUT(POF_MD2_FXOUT),
    POF_PRD_FXOUT(POF_PRD_FXOUT),
    POF_STD_FXOUT(POF_STD_FXOUT),
    POF_TD_FXOUT(POF_TD_FXOUT),
}


impl PayOffFunctionFXOUT {
    pub fn from_str(func: &str) -> PayOffFunctionFXOUT     {
        match func {
            "POF_AD_FXOUT" => Self::POF_AD_FXOUT(POF_AD_FXOUT::new()),
            // "POF_CD_FXOUT" => Self::POF_CD_FXOUT(POF_CD_FXOUT::new()),
            "POF_CE_FXOUT" => Self::POF_CE_FXOUT(POF_CE_FXOUT::new()),
            "POF_MD1_FXOUT" => Self::POF_MD1_FXOUT(POF_MD1_FXOUT::new()),
            "POF_MD2_FXOUT" => Self::POF_MD2_FXOUT(POF_MD2_FXOUT::new()),
            "POF_PRD_FXOUT" => Self::POF_PRD_FXOUT(POF_PRD_FXOUT::new()),
            "POF_STD_FXOUT" => Self::POF_STD_FXOUT(POF_STD_FXOUT::new()),
            "POF_TD_FXOUT" => Self::POF_TD_FXOUT(POF_TD_FXOUT::new()),
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
            PayOffFunctionFXOUT::POF_AD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            // PayOffFunctionFXOUT::POF_CD_FXOUT(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            PayOffFunctionFXOUT::POF_CE_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionFXOUT::POF_MD1_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionFXOUT::POF_MD2_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionFXOUT::POF_PRD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionFXOUT::POF_STD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionFXOUT::POF_TD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}
