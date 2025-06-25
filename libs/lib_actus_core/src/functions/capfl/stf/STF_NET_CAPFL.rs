use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;
use crate::events::ContractEvent::ContractEvent;

#[allow(non_camel_case_types)]
pub struct STF_NET_CAPFL {
    e1: ContractEvent,
    e2: ContractEvent,
}

impl STF_NET_CAPFL {
    pub fn new(e1: ContractEvent, e2: ContractEvent) -> Self {
        STF_NET_CAPFL { e1, e2 }
    }
}

impl TraitStateTransitionFunction for STF_NET_CAPFL {
    fn eval(
        &self,
        time: &IsoDatetime,
        _states: &StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> StateSpace {
        let mut post_event_states = StateSpace::new();
        post_event_states.statusDate = Some(*time);
        post_event_states
    }
}
