use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_calendar::BusinessDayConvention::BusinessDayConvention;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_TD_PAM;

impl TraitStateTransitionFunction for STF_TD_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayConvention,
    )  {

        // let mut new_states = StateSpace::copy_state_space(states);

        // Update state space
        states.notionalPrincipal = Some(0.0);
        states.accruedInterest = Some(0.0);
        states.feeAccrued = Some(0.0);
        states.statusDate = Some(*time);

        // Return a copy of the updated state space
    }
}
