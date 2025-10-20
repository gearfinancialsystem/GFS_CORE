use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::ann::StatesTransitionFunctionANN::StatesTransitionFunctionANN;
use crate::functions::lam::StatesTransitionFunctionLAM::StatesTransitionFunctionLAM;
use crate::functions::pam::StatesTransitionFunctionPAM::StatesTransitionFunctionPAM;
use crate::functions::fxout::StatesTransitionFunctionFXOUT::StatesTransitionFunctionFXOUT;
use crate::functions::nam::StatesTransitionFunctionNAM::StatesTransitionFunctionNAM;
use crate::functions::stk::StatesTransitionFunctionSTK::StatesTransitionFunctionSTK;
use crate::functions::swppv::StatesTransitionFunctionSWPPV::StatesTransitionFunctionSWPPV;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;

#[derive(Clone)]
pub enum StatesTransitionFunction {
    StatesTransitionFunctionPAM(StatesTransitionFunctionPAM),
    StatesTransitionFunctionLAM(StatesTransitionFunctionLAM),
    StatesTransitionFunctionFXOUT(StatesTransitionFunctionFXOUT),
    StatesTransitionFunctionSTK(StatesTransitionFunctionSTK),
    //StatesTransitionFunctionSWAPS(StatesTransitionFunctionSWAPS),
    StatesTransitionFunctionSWPPV(StatesTransitionFunctionSWPPV),
    StatesTransitionFunctionNAM(StatesTransitionFunctionNAM),
    StatesTransitionFunctionANN(StatesTransitionFunctionANN),
}

impl StatesTransitionFunction {
    pub fn from_str(func: &str) -> StatesTransitionFunction {
        let sub_str = func.split('_').last().unwrap_or("");

        match sub_str {
            "PAM" => Self::StatesTransitionFunctionPAM(StatesTransitionFunctionPAM::from_str(func)),
            "LAM" => Self::StatesTransitionFunctionLAM(StatesTransitionFunctionLAM::from_str(func)),
            "FXOUT" => Self::StatesTransitionFunctionFXOUT(StatesTransitionFunctionFXOUT::from_str(func)),
            "STK" => Self::StatesTransitionFunctionSTK(StatesTransitionFunctionSTK::from_str(func)),
            // "SWAPS" => Self::StatesTransitionFunctionSWAPS(StatesTransitionFunctionSWAPS::from_str(func)),
            "SWPPV" => Self::StatesTransitionFunctionSWPPV(StatesTransitionFunctionSWPPV::from_str(func)),
            "NAM" => Self::StatesTransitionFunctionNAM(StatesTransitionFunctionNAM::from_str(func)),
            "ANN" => Self::StatesTransitionFunctionANN(StatesTransitionFunctionANN::from_str(func)),
            _ => panic!("foirade")
        }
    }

    pub fn eval(&self, time: &PhantomIsoDatetimeW,
                states: &mut StatesSpace,
                contract_terms: &ContractTerms,
                contract_structure: &Option<RelatedContracts>,
                risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
                day_counter: &Option<DayCountConvention>,
                time_adjuster: &BusinessDayAdjuster,) {
        match self {
            Self::StatesTransitionFunctionPAM(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
            Self::StatesTransitionFunctionLAM(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
            Self::StatesTransitionFunctionFXOUT(v) => v.eval(time, states,
                                                           contract_terms,
                                                           contract_structure,
                                                           risk_factor_external_data,
                                                           day_counter,
                                                           time_adjuster),
            Self::StatesTransitionFunctionSTK(v) => v.eval(time, states,
                                                           contract_terms,
                                                           contract_structure,
                                                           risk_factor_external_data,
                                                           day_counter,
                                                           time_adjuster),
            // Self::StatesTransitionFunctionSWAPS(v) => v.eval(time, states,
            //                                                contract_terms,
            //                                                contract_structure,
            //                                                risk_factor_external_data,
            //                                                day_counter,
            //                                                time_adjuster),
            Self::StatesTransitionFunctionSWPPV(v) => v.eval(time, states,
                                                           contract_terms,
                                                           contract_structure,
                                                           risk_factor_external_data,
                                                           day_counter,
                                                           time_adjuster),
            Self::StatesTransitionFunctionANN(v) => v.eval(time, states,
                                                             contract_terms,
                                                             contract_structure,
                                                             risk_factor_external_data,
                                                             day_counter,
                                                             time_adjuster),
            Self::StatesTransitionFunctionNAM(v) => v.eval(time, states,
                                                           contract_terms,
                                                           contract_structure,
                                                           risk_factor_external_data,
                                                           day_counter,
                                                           time_adjuster),
        }
    }
}