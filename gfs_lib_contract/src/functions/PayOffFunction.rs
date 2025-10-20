use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::fxout::PayOffFunctionFXOUT::PayOffFunctionFXOUT;
use crate::functions::lam::PayOffFunctionLAM::PayOffFunctionLAM;
use crate::functions::pam::PayOffFunctionPAM::PayOffFunctionPAM;
use crate::functions::stk::PayOffFunctionSTK::PayOffFunctionSTK;
use crate::functions::swppv::PayOffFunctionSWPPV::PayOffFunctionSWPPV;
use crate::functions::ann::PayOffFunctionANN::PayOffFunctionANN;
use crate::functions::nam::PayOffFunctionNAM::PayOffFunctionNAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;

#[derive(Clone)]
pub enum PayOffFunction {
    PayOffFunctionPAM(PayOffFunctionPAM),
    PayOffFunctionLAM(PayOffFunctionLAM),
    PayOffFunctionFXOUT(PayOffFunctionFXOUT),
    PayOffFunctionSTK(PayOffFunctionSTK),
    PayOffFunctionSWPPV(PayOffFunctionSWPPV),
    PayOffFunctionANN(PayOffFunctionANN),
    PayOffFunctionNAM(PayOffFunctionNAM),
    // PayOffFunctionSWAPS(PayOffFunctionSWAPS),
}


impl PayOffFunction {
    pub fn from_str(func: &str) -> PayOffFunction {
        let sub_str = func.split('_').last().unwrap_or("");

        match sub_str {
            "PAM" => Self::PayOffFunctionPAM(PayOffFunctionPAM::from_str(func)),
            "LAM" => Self::PayOffFunctionLAM(PayOffFunctionLAM::from_str(func)),
            "FXOUT" => Self::PayOffFunctionFXOUT(PayOffFunctionFXOUT::from_str(func)),
            "STK" => Self::PayOffFunctionSTK(PayOffFunctionSTK::from_str(func)),
            "SWPPV" => Self::PayOffFunctionSWPPV(PayOffFunctionSWPPV::from_str(func)),
            "ANN" => Self::PayOffFunctionANN(PayOffFunctionANN::from_str(func)),
            "NAM" => Self::PayOffFunctionNAM(PayOffFunctionNAM::from_str(func)),
            //"SWAPS" => Self::PayOffFunctionSWAPS(PayOffFunctionSWAPS::from_str(func)),

            _ => panic!("foirade")
        }
    }
    
    pub fn eval(&self, time: &PhantomIsoDatetimeW,
                states: &StatesSpace,
                contract_terms: &ContractTerms,
                contract_structure: &Option<RelatedContracts>,
                risk_factor_external_data: &Option<Arc<dyn TraitExternalData>>,
                day_counter: &Option<DayCountConvention>,
                time_adjuster: &BusinessDayAdjuster,) -> f64 {
        match self { 
            Self::PayOffFunctionPAM(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
            Self::PayOffFunctionLAM(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
            Self::PayOffFunctionFXOUT(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
            Self::PayOffFunctionSTK(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
            // Self::PayOffFunctionSWAPS(v) => v.eval(time, states,
            //                                      contract_terms,
            //                                      contract_structure,
            //                                      risk_factor_external_data,
            //                                      day_counter,
            //                                      time_adjuster)
            Self::PayOffFunctionSWPPV(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
            Self::PayOffFunctionANN(v) => v.eval(time, states,
                                                   contract_terms,
                                                   contract_structure,
                                                   risk_factor_external_data,
                                                   day_counter,
                                                   time_adjuster),
            Self::PayOffFunctionNAM(v) => v.eval(time, states,
                                                 contract_terms,
                                                 contract_structure,
                                                 risk_factor_external_data,
                                                 day_counter,
                                                 time_adjuster),
        }
    }
}
