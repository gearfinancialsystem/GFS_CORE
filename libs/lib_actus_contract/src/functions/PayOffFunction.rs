use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
//use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::lam::PayOffFunctionLAM::PayOffFunctionLAM;
use crate::functions::pam::PayOffFunctionPAM::PayOffFunctionPAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;

#[derive(Clone)]
pub enum PayOffFunction {
    PayOffFunctionPAM(PayOffFunctionPAM),
    PayOffFunctionLAM(PayOffFunctionLAM)
}


impl PayOffFunction {
    pub fn from_str(func: &str) -> PayOffFunction {
        let sub_str = func.split('_').last().unwrap_or("");

        match sub_str {
            "PAM" => Self::PayOffFunctionPAM(PayOffFunctionPAM::from_str(func)),
            "LAM" => Self::PayOffFunctionLAM(PayOffFunctionLAM::from_str(func)),
            _ => panic!("foirade")
        }
    }
    
    pub fn eval(&self, time: &PhantomIsoDatetimeW,
                states: &StatesSpace,
                contract_terms: &ContractTerms,
                contract_structure: &Option<RelatedContracts>,
                risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
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
                                                 time_adjuster)
        }
    }
}
