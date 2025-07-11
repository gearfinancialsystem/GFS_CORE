use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_TD_BCS;

impl TraitStateTransitionFunction for STF_TD_BCS {
    fn eval(
        &self,
        _time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        states.notional_principal = NotionalPrincipal::new(0.0).ok();

    }
}
