#![allow(non_camel_case_types)]

use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::fxout::stf::STF_CD_FXOUT::STF_CD_FXOUT;
use crate::functions::fxout::stf::STF_CE_FXOUT::STF_CE_FXOUT;
use crate::functions::fxout::stf::STF_MD1_FXOUT::STF_MD1_FXOUT;
use crate::functions::fxout::stf::STF_MD2_FXOUT::STF_MD2_FXOUT;
use crate::functions::fxout::stf::STF_PRD_FXOUT::STF_PRD_FXOUT;
use crate::functions::fxout::stf::STF_STD_FXOUT::STF_STD_FXOUT;
use crate::functions::fxout::stf::STF_TD_FXOUT::STF_TD_FXOUT;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionFXOUT {

    // STF_AD_FXOUT(STF_AD_FXOUT),
    STF_CD_FXOUT(STF_CD_FXOUT),
    STF_CE_FXOUT(STF_CE_FXOUT),
    STF_MD1_FXOUT(STF_MD1_FXOUT),
    STF_MD2_FXOUT(STF_MD2_FXOUT),
    STF_PRD_FXOUT(STF_PRD_FXOUT),
    STF_STD_FXOUT(STF_STD_FXOUT),
    STF_TD_FXOUT(STF_TD_FXOUT),
}

impl StatesTransitionFunctionFXOUT {
    pub fn from_str(func: &str) -> StatesTransitionFunctionFXOUT {
        match func {
            // "STF_AD_FXOUT" => Self::STF_AD_FXOUT(STF_AD_FXOUT::new()),
            "STF_CD_FXOUT" => Self::STF_CD_FXOUT(STF_CD_FXOUT::new()),
            "STF_CE_FXOUT" => Self::STF_CE_FXOUT(STF_CE_FXOUT::new()),
            "STF_MD1_FXOUT" => Self::STF_MD1_FXOUT(STF_MD1_FXOUT::new()),
            "STF_MD2_FXOUT" => Self::STF_MD2_FXOUT(STF_MD2_FXOUT::new()),
            "STF_PRD_FXOUT" => Self::STF_PRD_FXOUT(STF_PRD_FXOUT::new()),
            "STF_STD_FXOUT" => Self::STF_STD_FXOUT(STF_STD_FXOUT::new()),
            "STF_TD_FXOUT" => Self::STF_TD_FXOUT(STF_TD_FXOUT::new()),
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
            // StatesTransitionFunctionPAM::STF_AD_FXOUT(v) => v.eval(
            //     time, states, contract_terms, contract_structure,
            //     risk_factor_external_data, day_counter, time_adjuster
            // ),
            StatesTransitionFunctionFXOUT::STF_CD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionFXOUT::STF_CE_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionFXOUT::STF_MD1_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionFXOUT::STF_MD2_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionFXOUT::STF_PRD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionFXOUT::STF_STD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionFXOUT::STF_TD_FXOUT(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}