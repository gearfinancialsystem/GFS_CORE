use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_terms::ContractTerms::{ContractTerms};

use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
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
        _model: &ContractTerms,
        _risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        self.e1.clone().unwrap().payoff.unwrap() + self.e2.clone().unwrap().payoff.unwrap()
    }
}