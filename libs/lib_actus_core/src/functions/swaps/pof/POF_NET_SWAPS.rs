use crate::events::ContractEvent::ContractEvent;
use crate::attributes::ContractTerms::{ContractTerms};

use crate::external::RiskFactorModel::RiskFactorModel;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct POF_NET_SWAPS {
    pub e1: Option<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub e2: Option<ContractEvent<IsoDatetime, IsoDatetime>>,
}

impl POF_NET_SWAPS {
    pub fn new(e1: ContractEvent<IsoDatetime, IsoDatetime>, e2: ContractEvent<IsoDatetime, IsoDatetime>) -> Self {
        POF_NET_SWAPS { e1: Some(e1), e2: Some(e2) }
    }
}

impl TraitPayOffFunction for POF_NET_SWAPS {
    fn eval(
        &self,
        _time: &IsoDatetime,
        _states: &StatesSpace,
        _contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        _risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        self.e1.clone().unwrap().payoff.unwrap() + self.e2.clone().unwrap().payoff.unwrap()
    }
}