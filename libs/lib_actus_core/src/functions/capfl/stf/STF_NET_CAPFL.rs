use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
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
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    )  {
        states.statusDate = Some(*time);
    }
}
