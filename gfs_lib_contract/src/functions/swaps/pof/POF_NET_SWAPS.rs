use crate::events::ContractEvent::ContractEvent;
use crate::attributes::ContractTerms::{ContractTerms};
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

#[allow(non_camel_case_types)]
pub struct POF_NET_SWAPS {
    pub e1: Option<ContractEvent<PhantomIsoDatetimeW, PhantomIsoDatetimeW>>,
    pub e2: Option<ContractEvent<PhantomIsoDatetimeW, PhantomIsoDatetimeW>>,
}

impl POF_NET_SWAPS {
    pub fn new(e1: ContractEvent<PhantomIsoDatetimeW, PhantomIsoDatetimeW>, e2: ContractEvent<PhantomIsoDatetimeW, PhantomIsoDatetimeW>) -> Self {
        POF_NET_SWAPS { e1: Some(e1), e2: Some(e2) }
    }
}

impl TraitPayOffFunction for POF_NET_SWAPS {
    fn eval(
        &self,
        _time: &PhantomIsoDatetimeW,
        _states: &StatesSpace,
        _contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<impl TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        self.e1.clone().unwrap().payoff.unwrap() + self.e2.clone().unwrap().payoff.unwrap()
    }
}