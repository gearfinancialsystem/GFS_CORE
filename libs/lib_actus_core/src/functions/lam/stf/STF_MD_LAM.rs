use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::state_space::StateSpace::StateSpace;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_MD_LAM;

impl TraitStateTransitionFunction for STF_MD_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        _model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        _time_adjuster: &BusinessDayAdjuster,
    ) {
        // Create a mutable copy of the states to update


        // Update state space
        states.notionalPrincipal = 0.0;
        states.accruedInterest = 0.0;
        states.feeAccrued = 0.0;
        states.interestCalculationBaseAmount = 0.0;
        states.statusDate = Some(*time);


    }
}
