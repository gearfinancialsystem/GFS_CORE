use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_NET_SWAPS {
    pub e1: Option<ContractEvent>,
    pub e2: Option<ContractEvent>,
}

impl STF_NET_SWAPS {
    pub fn new(e1: ContractEvent, e2: ContractEvent) -> Self {
        STF_NET_SWAPS { e1: Some(e1), e2: Some(e2) }
    }
}

impl TraitStateTransitionFunction for STF_NET_SWAPS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        let e1_states = self.e1.clone().unwrap().states();
        let e2_states = self.e2.clone().unwrap().states();

        let notional_principal_e1 = e1_states.notionalPrincipal.expect("should be some");
        let notional_principal_e2 = e2_states.notionalPrincipal.expect("should be some");
        let accrued_interest_e1 = e1_states.accruedInterest.expect("should be some");
        let accrued_interest_e2 = e2_states.accruedInterest.expect("should be some");

        states.notionalPrincipal = Some(notional_principal_e1 + notional_principal_e2);
        states.accruedInterest = Some(accrued_interest_e1 + accrued_interest_e2);

        states.statusDate = Some(*time);
    }
}