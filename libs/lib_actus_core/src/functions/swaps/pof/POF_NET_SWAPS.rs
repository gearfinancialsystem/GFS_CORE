use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::isoDatetime::IsoDatetime;

pub struct POF_NET_SWAPS {
    pub e1: Option<ContractEvent>,
    pub e2: Option<ContractEvent>,
}

impl POF_NET_SWAPS {
    pub fn new(e1: ContractEvent, e2: ContractEvent) -> Self {
        POF_NET_SWAPS { e1: Some(e1), e2: Some(e2) }
    }
}

impl TraitPayOffFunction for POF_NET_SWAPS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        self.e1.clone().unwrap().payoff.unwrap() + self.e2.clone().unwrap().payoff.unwrap()
    }
}